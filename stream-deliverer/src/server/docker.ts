import { spawn } from 'child_process';

export class Tag {
    public name: string;
    public version: string;

    constructor(name: string, version: string) {
        this.name = name;
        this.version = version;
    }

    toString(): string {
        return `${this.name}:${this.version}`;
    }

    clone(): Tag {
        return new Tag(this.name, this.version);
    }
}

export class DockerFile {
    private location: string;
    private tag: Tag;
    private context: string;

    constructor(location: string, tag: Tag, context: string) {
        this.location = location;
        this.tag = tag;
        this.context = context;
    }

    getLocation(): string {
        return this.location;
    }

    getTag(): Tag {
        return this.tag.clone();
    }

    getContext(): string {
        return this.context;
    }

    async build(): Promise<Image> {
        const cmd = `docker build -t ${this.tag.name} -f ${this.location} ${this.context}`;

        return new Promise((resolve, reject) => {
            const process = spawn(cmd);

            process.on('error', (error) => {
                reject(new Error(`Failed to start docker build command: ${error.message}`));
            });

            process.on('close', (code) => {
                if (code === 0) {
                    resolve(new Image(this.tag.clone()));
                } else {
                    reject(new Error(`Docker build failed with exit code ${code}`));
                }
            });
        });
    }

    clone(): DockerFile {
        return new DockerFile(this.location, this.tag.clone(), this.context);
    }
}

export class Image {
    private tag: Tag;

    constructor(tag: Tag) {
        this.tag = tag;
    }

    getTag(): Tag {
        return this.tag.clone();
    }

    clone(): Image {
        return new Image(this.tag.clone());
    }
}

export enum Status {
    Running = "Running",
    Stopped = "Stopped",
    Exited = "Exited"
}

// Simple async lock implementation
class AsyncLock<T> {
    private value: T;
    private locked = false;
    private queue: Array<() => void> = [];

    constructor(initialValue: T) {
        this.value = initialValue;
    }

    async lock(): Promise<AsyncLockGuard<T>> {
        while (this.locked) {
            await new Promise<void>(resolve => {
                this.queue.push(resolve);
            });
        }
        this.locked = true;
        return new AsyncLockGuard(this);
    }

    unlock(): void {
        this.locked = false;
        const next = this.queue.shift();
        if (next) {
            next();
        }
    }

    getValue(): T {
        return this.value;
    }

    setValue(value: T): void {
        this.value = value;
    }
}

class AsyncLockGuard<T> {
    private lock: AsyncLock<T>;

    constructor(lock: AsyncLock<T>) {
        this.lock = lock;
    }

    get value(): T {
        return this.lock.getValue();
    }

    set value(val: T) {
        this.lock.setValue(val);
    }

    release(): void {
        this.lock.unlock();
    }
}

export class Container {
    private name: string;
    private image: Image;
    private ports: number[] = [];
    private status: AsyncLock<Status>;

    constructor(name: string, image: Image) {
        this.name = name;
        this.image = image;
        this.status = new AsyncLock(Status.Stopped);
    }

    getPorts(): number[] {
        return [...this.ports];
    }

    getName(): string {
        return this.name;
    }

    getImage(): Image {
        return this.image.clone();
    }

    async getStatus(): Promise<Status> {
        const guard = await this.status.lock();
        const status = guard.value;
        guard.release();
        return status;
    }

    addPort(port: number): void {
        this.ports.push(port);
    }

    removePort(port: number): void {
        this.ports = this.ports.filter(p => p !== port);
    }

    async start(): Promise<void> {
        const guard = await this.status.lock();

        if (guard.value === Status.Running) {
            guard.release();
            return;
        }

        guard.value = Status.Running;

        const portMappings = this.ports
            .map(port => `-p ${port}:${port}`)
            .join(' ');

        const cmd = `docker run --rm --name ${this.name} ${portMappings} ${this.image.getTag()}`;

        return new Promise((resolve, reject) => {
            const process = spawn(cmd);

            process.on('error', (error) => {
                guard.release();
                reject(new Error(`Failed to start docker run command: ${error.message}`));
            });

            process.on('spawn', () => {
                console.log(`Started container: ${this.name}`);
                guard.release();
                resolve();
            });
        });
    }

    async stop(): Promise<void> {
        const guard = await this.status.lock();

        if (guard.value !== Status.Running) {
            guard.release();
            return;
        }

        guard.value = Status.Stopped;

        const cmd = `docker stop ${this.name}`;

        return new Promise((resolve, reject) => {
            const process = spawn(cmd);

            process.on('error', (error) => {
                guard.release();
                reject(new Error(`Failed to start docker stop command: ${error.message}`));
            });

            process.on('close', (code) => {
                console.log(`Stopped container: ${this.name}`);
                guard.release();
                if (code === 0) {
                    resolve();
                } else {
                    reject(new Error(`Docker stop failed with exit code ${code}`));
                }
            });
        });
    }

    clone(): Container {
        const cloned = new Container(this.name, this.image.clone());
        cloned.ports = [...this.ports];
        return cloned;
    }
}