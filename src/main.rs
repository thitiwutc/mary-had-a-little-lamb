use mary_had_a_little_lamb::{self, A4, B4, D5, G4, HALF, Note, QUARTER, WHOLE};
use std::time::Duration;

fn main() {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // หนูมาลี (Mary had a little lamb)
    let notes = [
        Note::new(B4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(G4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(B4, QUARTER),
        Note::new(B4, QUARTER),
        Note::new(B4, HALF),
        Note::new(A4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(A4, HALF),
        Note::new(B4, QUARTER),
        Note::new(D5, QUARTER),
        Note::new(D5, HALF),
        Note::new(B4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(G4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(B4, QUARTER),
        Note::new(B4, QUARTER),
        Note::new(B4, HALF),
        Note::new(A4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(B4, QUARTER),
        Note::new(A4, QUARTER),
        Note::new(G4, WHOLE),
    ];

    let whole_note_dur = Duration::from_secs_f32(1.2);
    let dur_between_notes = Duration::from_secs_f32(0.15);
    Note::play(&sink, &notes, whole_note_dur, dur_between_notes);
}
