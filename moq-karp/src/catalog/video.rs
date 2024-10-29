use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, DisplayFromStr};

use super::{Dimensions, Track, VideoCodec};

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
	// Generic information about the track
	pub track: Track,

	// The codec mimetype encoded as a string
	// ex. avc1.42c01e
	#[serde_as(as = "DisplayFromStr")]
	pub codec: VideoCodec,

	// Some codecs unfortunately aren't self-describing
	// One of the best examples is H264, which needs the sps/pps out of band to function.
	#[serde(default, skip_serializing_if = "Bytes::is_empty")]
	#[serde_as(as = "Hex")]
	pub description: Bytes,

	// The encoded width/height of the media
	pub resolution: Dimensions,

	#[serde(default)]
	pub bitrate: Option<u32>,
}
