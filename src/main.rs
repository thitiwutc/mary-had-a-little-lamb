use mary_had_a_little_lamb::*;
use std::time::Duration;

fn main() {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // หนูมาลี (Mary had a little lamb)
    let notes = [
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(G4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(B4, HALF)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(A4, HALF)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(D5, QUARTER)),
        Sound::Note(Note::new(D5, HALF)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(G4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(B4, HALF)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(B4, QUARTER)),
        Sound::Note(Note::new(A4, QUARTER)),
        Sound::Note(Note::new(G4, WHOLE)),
    ];

    let whole_note_dur = Duration::from_secs_f32(1.2);
    let dur_between_notes = Duration::from_secs_f32(0.15);
    Note::play(&sink, &notes, whole_note_dur, dur_between_notes);
}
