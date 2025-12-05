# mary-had-a-little-lamb

A CLI app that plays Mary Had a Little Lamb song

To start the app, run:

```
cargo run
```

## Other songs

**เพลงช้าง**
```rust
    let notes = [
        Sound::Note(Note::new(G4, QUARTER)), // ช้าง
        Sound::Note(Note::new(G4, QUARTER)), // ช้าง
        Sound::Note(Note::new(G4, QUARTER)), // ช้าง
        Sound::Rest(EIGHTH),
        Sound::Note(Note::new(G4, EIGHTH)),  // น้อง
        Sound::Note(Note::new(E4, EIGHTH)),  // เคย
        Sound::Note(Note::new(D4, EIGHTH)),  // เห็น
        Sound::Note(Note::new(G4, EIGHTH)),  // ข้าง
        Sound::Note(Note::new(G4, EIGHTH)),  // หรือ
        Sound::Note(Note::new(C4, QUARTER)), // เปล่า
        Sound::Rest(EIGHTH),
        Sound::Note(Note::new(G4, EIGHTH)), // ช้าง
        Sound::Note(Note::new(E4, EIGHTH)), // มัน
        Sound::Note(Note::new(D4, EIGHTH)), // ตัว
        Sound::Note(Note::new(E4, EIGHTH)), // โต
        Sound::Note(Note::new(C4, EIGHTH)), // ไม่
        Sound::Note(Note::new(D4, EIGHTH)), // เบา
        Sound::Note(Note::new(C4, EIGHTH)), // จ
        Sound::Note(Note::new(A3, EIGHTH)), // มูก
        Sound::Note(Note::new(C4, EIGHTH)), // ยาว
        Sound::Note(Note::new(C4, EIGHTH)), // ยาว
        Sound::Note(Note::new(A3, EIGHTH)), // เรียก
        Sound::Note(Note::new(F3, EIGHTH)), // ว่า
        Sound::Note(Note::new(C4, EIGHTH)), // งวง
        Sound::Rest(EIGHTH),
        Sound::Note(Note::new(C4, EIGHTH)), // มี
        Sound::Note(Note::new(A3, EIGHTH)), // เขี้ยว
        Sound::Note(Note::new(F3, EIGHTH)), // ใต้
        Sound::Note(Note::new(C4, EIGHTH)), // งวง
        Sound::Note(Note::new(A3, EIGHTH)), // เรียก
        Sound::Note(Note::new(F3, EIGHTH)), // ว่า
        Sound::Note(Note::new(C4, EIGHTH)), // งา
        Sound::Rest(EIGHTH),
        Sound::Note(Note::new(G4, EIGHTH)),  // มี
        Sound::Note(Note::new(A4, EIGHTH)),  // หู
        Sound::Note(Note::new(G4, EIGHTH)),  // มี
        Sound::Note(Note::new(E4, QUARTER)), // ตา
        Sound::Note(Note::new(D4, QUARTER)), // หาง
        Sound::Note(Note::new(C4, HALF)),    // ยาว
    ];
```
