use mary_had_a_little_lamb::*;
use std::time::Duration;

fn main() {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // หนูมาลี (Mary had a little lamb)
    let notes = [
        Sound::Note(Note::new(B4, QUARTER)), // หนู
        Sound::Note(Note::new(A4, QUARTER)), // มา
        Sound::Note(Note::new(G4, QUARTER)), // ลี
        Sound::Note(Note::new(A4, QUARTER)), // มี
        Sound::Note(Note::new(B4, QUARTER)), // ลูก
        Sound::Note(Note::new(B4, QUARTER)), // แมว
        Sound::Note(Note::new(B4, HALF)),    // เหมียว
        Sound::Note(Note::new(A4, QUARTER)), // ลูก
        Sound::Note(Note::new(A4, QUARTER)), // แมว
        Sound::Note(Note::new(A4, HALF)),    // เหมียว
        Sound::Note(Note::new(B4, QUARTER)), // ลูก
        Sound::Note(Note::new(D5, QUARTER)), // แมว
        Sound::Note(Note::new(D5, HALF)),    // เหมียว
        Sound::Note(Note::new(B4, QUARTER)), // หนู
        Sound::Note(Note::new(A4, QUARTER)), // มา
        Sound::Note(Note::new(G4, QUARTER)), // ลี
        Sound::Note(Note::new(A4, QUARTER)), // มี
        Sound::Note(Note::new(B4, QUARTER)), // ลูก
        Sound::Note(Note::new(B4, QUARTER)), // แมว
        Sound::Note(Note::new(B4, HALF)),    // เหมียว
        Sound::Note(Note::new(A4, QUARTER)), // ขน
        Sound::Note(Note::new(A4, QUARTER)), // มัน
        Sound::Note(Note::new(B4, QUARTER)), // คล้าย
        Sound::Note(Note::new(A4, QUARTER)), // สำ
        Sound::Note(Note::new(G4, WHOLE)),   // ลี
    ];

    let whole_note_dur = Duration::from_secs_f32(1.2);
    let dur_between_notes = Duration::from_secs_f32(0.15);
    Note::play(&sink, &notes, whole_note_dur, dur_between_notes);
}
