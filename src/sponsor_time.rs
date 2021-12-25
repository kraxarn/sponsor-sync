use uuid::Uuid;

pub struct SponsorTime {
	pub id: Uuid,
	pub video_id: String,
	pub start_time: f32,
	pub end_time: f32,
}