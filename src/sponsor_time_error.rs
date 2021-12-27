pub enum SponsorTimeError {
	/// Time contained an invalid interval
	InvalidInterval,
	/// Database returned an error
	DatabaseError(Option<String>),
}