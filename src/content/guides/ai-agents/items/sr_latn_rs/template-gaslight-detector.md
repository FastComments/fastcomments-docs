**Template ID:** `gaslight_detector`

Gaslight Detector prati izmene komentara koje prepisuju istoriju usred konverzacije - one gde autor menja značenje ranijeg komentara nakon što su odgovori već napisani, ostavljajući naknadne odgovore van konteksta ili pogrešne. Kada agent odluči da izmena prelazi tu granicu, on vraća originalni tekst i šalje autoru direktnu poruku da objasni.

Ovo je šablon sa većim rizikom jer menja korisnički sadržaj. Pokrenite ga u režimu [dry-run](#dry-run-mode) duže nego što biste to uradili za šablon samo za čitanje, i stavite `edit_comment` iza [odobrenja](#approval-workflow) dok ne poverite prosudbi modela na vašem saobraćaju.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - agent upoređuje novi i prethodni tekst i odlučuje da li izmena izobličava odgovore koji već postoje.

Pogledajte [Trigger: Comment Edited](#trigger-comment-edit) za potpuni payload, uključujući prethodni tekst komentara i broj odgovora u trenutku izmene.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - koristi se za vraćanje originalnog teksta kada se proceni da je izmena gaslighting.
- [`warn_user`](#tool-warn-user) - izdaje blago upozorenje koje korisnik vidi pri sledećoj poseti.
- [`send_dm`](#tools-overview) - kanal objašnjenja; korisnik dobija direktnu poruku koja opisuje zašto je njihova izmena poništena.

Ne može banovati, označavati kao spam, glasati ili objavljivati nove komentare - pokrivenost je namerno ograničena.

### Recommended additions before going live

- **Stavite `edit_comment` iza [odobrenja](#approval-workflow).** Vraćanje komentara je vidljivo autoru i svima koji su videli izmenjenu verziju, tako da je lažno pozitivan rezultat neprijatan. Držite odobrenja uključena dok dry-run ne pokaže da je agent dosledan.
- **Precizirajte prompt sa primerima šta se na vašem sajtu računa kao gaslighting.** Podrazumevani prompt je kratak namerno. Dajte modelu konkretne primere - "promena tvrdnje iz da/ne", "brisanje broja na koji se odgovori pozivaju", "dodavanje neprijateljske rečenice nakon što su odgovori već postavljeni" - i eksplicitne ne-primere kao što su ispravke tipografskih grešaka, formatiranje ili dodavanje izvora.
- **Koristite broj odgovora iz konteksta okidača.** Izmene komentara sa nula odgovora ne mogu izobličiti konverzaciju; prompt bi trebalo da kaže modelu da preskoči takve izmene.
- **Označite "Include commenter's trust factor, account age, ban history, and recent comments"** u [Context Options](#context-options). Model je znatno manje agresivan kada može da vidi dugogodišnji nalog koji radi u dobroj veri.
- **Razmislite o kratkom periodu milosti za izmene u promptu.** Mnoge izmene u prvih 30–60 sekundi su ispravke tipa; uputite model da ignoriše izmene te brze.

### Recommended dry-run window

Pokrećite najmanje dve nedelje sa stvarnim saobraćajem u režimu [dry-run](#dry-run-mode) pre nego što pređete na Enabled, i pregledajte svaku označenu izmenu tokom tog perioda. Koristite [Test Runs (Replays)](#test-runs-replays) da replay-ujete poslednjih 30 dana izmena protiv agenta pre puštanja u rad.