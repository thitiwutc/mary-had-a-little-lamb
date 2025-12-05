use std::{ops::Add, thread, time::Duration};

use rodio::{Sink, Source, source::SineWave};

pub struct NoteDuration(pub f32);

pub const WHOLE: NoteDuration = NoteDuration(1.0);
pub const HALF: NoteDuration = NoteDuration(1.0 / 2.0);
pub const QUARTER: NoteDuration = NoteDuration(1.0 / 4.0);
pub const EIGHTH: NoteDuration = NoteDuration(1.0 / 8.0);
pub const SIXTEENTH: NoteDuration = NoteDuration(1.0 / 16.0);
pub const THIRTY_SECOND: NoteDuration = NoteDuration(1.0 / 32.0);
pub const SIXTY_FOURTH: NoteDuration = NoteDuration(1.0 / 64.0);

pub struct NoteFrequency(f32);

impl Into<f32> for NoteFrequency {
    fn into(self) -> f32 {
        self.0
    }
}

// Octave 0
pub const C0: NoteFrequency = NoteFrequency(16.35);
pub const CS0: NoteFrequency = NoteFrequency(17.32);
pub const D0: NoteFrequency = NoteFrequency(18.35);
pub const DS0: NoteFrequency = NoteFrequency(19.45);
pub const E0: NoteFrequency = NoteFrequency(20.60);
pub const F0: NoteFrequency = NoteFrequency(21.83);
pub const FS0: NoteFrequency = NoteFrequency(23.12);
pub const G0: NoteFrequency = NoteFrequency(24.50);
pub const GS0: NoteFrequency = NoteFrequency(25.96);
pub const A0: NoteFrequency = NoteFrequency(27.50);
pub const AS0: NoteFrequency = NoteFrequency(29.14);
pub const B0: NoteFrequency = NoteFrequency(30.87);

// Octave 1
pub const C1: NoteFrequency = NoteFrequency(32.70);
pub const CS1: NoteFrequency = NoteFrequency(34.65);
pub const D1: NoteFrequency = NoteFrequency(36.71);
pub const DS1: NoteFrequency = NoteFrequency(38.89);
pub const E1: NoteFrequency = NoteFrequency(41.20);
pub const F1: NoteFrequency = NoteFrequency(43.65);
pub const FS1: NoteFrequency = NoteFrequency(46.25);
pub const G1: NoteFrequency = NoteFrequency(49.00);
pub const GS1: NoteFrequency = NoteFrequency(51.91);
pub const A1: NoteFrequency = NoteFrequency(55.00);
pub const AS1: NoteFrequency = NoteFrequency(58.27);
pub const B1: NoteFrequency = NoteFrequency(61.74);

// Octave 2
pub const C2: NoteFrequency = NoteFrequency(65.41);
pub const CS2: NoteFrequency = NoteFrequency(69.30);
pub const D2: NoteFrequency = NoteFrequency(73.42);
pub const DS2: NoteFrequency = NoteFrequency(77.78);
pub const E2: NoteFrequency = NoteFrequency(82.41);
pub const F2: NoteFrequency = NoteFrequency(87.31);
pub const FS2: NoteFrequency = NoteFrequency(92.50);
pub const G2: NoteFrequency = NoteFrequency(98.00);
pub const GS2: NoteFrequency = NoteFrequency(103.83);
pub const A2: NoteFrequency = NoteFrequency(110.00);
pub const AS2: NoteFrequency = NoteFrequency(116.54);
pub const B2: NoteFrequency = NoteFrequency(123.47);

// Octave 3
pub const C3: NoteFrequency = NoteFrequency(130.81);
pub const CS3: NoteFrequency = NoteFrequency(138.59);
pub const D3: NoteFrequency = NoteFrequency(146.83);
pub const DS3: NoteFrequency = NoteFrequency(155.56);
pub const E3: NoteFrequency = NoteFrequency(164.81);
pub const F3: NoteFrequency = NoteFrequency(174.61);
pub const FS3: NoteFrequency = NoteFrequency(185.00);
pub const G3: NoteFrequency = NoteFrequency(196.00);
pub const GS3: NoteFrequency = NoteFrequency(207.65);
pub const A3: NoteFrequency = NoteFrequency(220.00);
pub const AS3: NoteFrequency = NoteFrequency(233.08);
pub const B3: NoteFrequency = NoteFrequency(246.94);

// Octave 4
pub const C4: NoteFrequency = NoteFrequency(261.63);
pub const CS4: NoteFrequency = NoteFrequency(277.18);
pub const D4: NoteFrequency = NoteFrequency(293.66);
pub const DS4: NoteFrequency = NoteFrequency(311.13);
pub const E4: NoteFrequency = NoteFrequency(329.63);
pub const F4: NoteFrequency = NoteFrequency(349.23);
pub const FS4: NoteFrequency = NoteFrequency(369.99);
pub const G4: NoteFrequency = NoteFrequency(392.00);
pub const GS4: NoteFrequency = NoteFrequency(415.30);
pub const A4: NoteFrequency = NoteFrequency(440.00);
pub const AS4: NoteFrequency = NoteFrequency(466.16);
pub const B4: NoteFrequency = NoteFrequency(493.88);

// Octave 5
pub const C5: NoteFrequency = NoteFrequency(523.25);
pub const CS5: NoteFrequency = NoteFrequency(554.37);
pub const D5: NoteFrequency = NoteFrequency(587.33);
pub const DS5: NoteFrequency = NoteFrequency(622.25);
pub const E5: NoteFrequency = NoteFrequency(659.25);
pub const F5: NoteFrequency = NoteFrequency(698.46);
pub const FS5: NoteFrequency = NoteFrequency(739.99);
pub const G5: NoteFrequency = NoteFrequency(783.99);
pub const GS5: NoteFrequency = NoteFrequency(830.61);
pub const A5: NoteFrequency = NoteFrequency(880.00);
pub const AS5: NoteFrequency = NoteFrequency(932.33);
pub const B5: NoteFrequency = NoteFrequency(987.77);

// Octave 6
pub const C6: NoteFrequency = NoteFrequency(1046.50);
pub const CS6: NoteFrequency = NoteFrequency(1108.73);
pub const D6: NoteFrequency = NoteFrequency(1174.66);
pub const DS6: NoteFrequency = NoteFrequency(1244.51);
pub const E6: NoteFrequency = NoteFrequency(1318.51);
pub const F6: NoteFrequency = NoteFrequency(1396.91);
pub const FS6: NoteFrequency = NoteFrequency(1479.98);
pub const G6: NoteFrequency = NoteFrequency(1567.98);
pub const GS6: NoteFrequency = NoteFrequency(1661.22);
pub const A6: NoteFrequency = NoteFrequency(1760.00);
pub const AS6: NoteFrequency = NoteFrequency(1864.66);
pub const B6: NoteFrequency = NoteFrequency(1975.53);

// Octave 7
pub const C7: NoteFrequency = NoteFrequency(2093.00);
pub const CS7: NoteFrequency = NoteFrequency(2217.46);
pub const D7: NoteFrequency = NoteFrequency(2349.32);
pub const DS7: NoteFrequency = NoteFrequency(2489.02);
pub const E7: NoteFrequency = NoteFrequency(2637.02);
pub const F7: NoteFrequency = NoteFrequency(2793.83);
pub const FS7: NoteFrequency = NoteFrequency(2959.96);
pub const G7: NoteFrequency = NoteFrequency(3135.96);
pub const GS7: NoteFrequency = NoteFrequency(3322.44);
pub const A7: NoteFrequency = NoteFrequency(3520.00);
pub const AS7: NoteFrequency = NoteFrequency(3729.31);
pub const B7: NoteFrequency = NoteFrequency(3951.07);

// Octave 8
pub const C8: NoteFrequency = NoteFrequency(4186.01);
pub const CS8: NoteFrequency = NoteFrequency(4434.92);
pub const D8: NoteFrequency = NoteFrequency(4698.63);
pub const DS8: NoteFrequency = NoteFrequency(4978.03);
pub const E8: NoteFrequency = NoteFrequency(5274.04);
pub const F8: NoteFrequency = NoteFrequency(5587.65);
pub const FS8: NoteFrequency = NoteFrequency(5919.91);
pub const G8: NoteFrequency = NoteFrequency(6271.93);
pub const GS8: NoteFrequency = NoteFrequency(6644.88);
pub const A8: NoteFrequency = NoteFrequency(7040.00);
pub const AS8: NoteFrequency = NoteFrequency(7458.62);
pub const B8: NoteFrequency = NoteFrequency(7902.13);

pub enum Sound {
    Note(Note),
    Rest(NoteDuration),
}

pub struct Note {
    pub freq: NoteFrequency,
    pub dur: NoteDuration,
}

impl Note {
    pub fn new(freq: NoteFrequency, dur: NoteDuration) -> Note {
        Note { freq, dur }
    }

    pub fn play(
        sink: &Sink,
        notes: &[Sound],
        whole_note_dur: Duration,
        dur_between_notes: Duration,
    ) {
        for note in notes {
            match note {
                Sound::Note(note) => {
                    let note_dur = whole_note_dur.mul_f32(note.dur.0);
                    let source = SineWave::new(note.freq.0).take_duration(note_dur);

                    sink.append(source);
                    thread::sleep(note_dur.add(dur_between_notes));
                }
                Sound::Rest(rest_dur) => {
                    let note_dur = whole_note_dur.mul_f32(rest_dur.0);
                    thread::sleep(note_dur.add(dur_between_notes));
                }
            };
        }

        // Block the main thread until the Sink has played all appended sources.
        sink.sleep_until_end();
    }
}
