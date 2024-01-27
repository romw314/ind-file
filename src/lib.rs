use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use indicatif::ProgressBar;

pub fn write_with_progress(pb: &mut ProgressBar, bytes: &[u8], file_path: PathBuf, chunk_size: usize) -> anyhow::Result<()> {
	pb.set_length(bytes.len().try_into()?);
	pb.set_position(0);
	let mut file = File::create(file_path)?;
	for chunk in bytes.chunks(chunk_size) {
		file.write_all(chunk)?;
		pb.inc(chunk.len().try_into()?);
	}
	Ok(())
}
