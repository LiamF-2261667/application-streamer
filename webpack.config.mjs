import path from "node:path";
import { fileURLToPath } from "node:url";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import CopyPlugin from "copy-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import nodeExternals from "webpack-node-externals";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const config = [
	// Browser-specific configuration
	{
		entry: {
			index: "./stream-deliverer/src/pages/index.ts",
			setup: "./stream-deliverer/src/pages/setup.ts",
		},
		output: {
			path: path.resolve(__dirname, "dist"),
			filename: "[name].js",
		},
		target: "web", // Target browser environment
		plugins: [
			new WasmPackPlugin({
				crateDirectory: path.resolve(__dirname, "stream-deliverer"),
				outDir: path.resolve(__dirname, "dist"),
				args: "--log-level warn",
				outName: "rust",
			}),
			new HtmlWebpackPlugin({
				template: "stream-deliverer/src/pages/index.html",
				filename: "index.html",
				chunks: ["index"],
			}),
			new HtmlWebpackPlugin({
				template: "stream-deliverer/src/pages/setup.html",
				filename: "setup.html",
				chunks: ["setup"],
			}),
			new MiniCssExtractPlugin(),
			new CopyPlugin({
				patterns: [
					{
						from: path.resolve(__dirname, "node_modules/@shoelace-style/shoelace/dist/assets"),
						to: path.resolve(__dirname, "dist/assets"),
					},
				],
			}),
		],
		mode: "development",
		experiments: {
			asyncWebAssembly: true,
			topLevelAwait: true,
		},
		module: {
			rules: [
				{
					test: /\.ts(x)?$/,
					loader: "ts-loader",
					exclude: /node_modules/,
				},
				{
					test: /\.css$/i,
					use: [MiniCssExtractPlugin.loader, "css-loader"],
				},
			],
		},
		resolve: {
			extensions: [".ts", ".tsx", ".js"],
			alias: {
				"@rust": path.resolve(__dirname, "dist", "rust"),
			},
		},
		devServer: {
			open: true,
			hot: false,
			liveReload: false,
			headers: {
				"Access-Control-Allow-Origin": "*", // Enable CORS
			},
		},
		optimization: {
			sideEffects: true,
		},
	},
];

export default config;