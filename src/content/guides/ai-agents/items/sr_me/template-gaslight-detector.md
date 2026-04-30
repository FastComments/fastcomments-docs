**ID predloška:** `gaslight_detector`

Detektor gaslight-a prati izmjene komentara koje prepravljaju istoriju usred razgovora — one u kojima autor promijeni značenje ranijeg komentara nakon što su odgovori već napisani, ostavljajući naknadne odgovore van konteksta ili netačne. Kada agent odluči da izmjena prelazi tu granicu, on vraća originalni tekst i pošalje autoru direktnu poruku objašnjavajući razloge.

Ovo je predložak višeg rizika zato što mijenja korisnički sadržaj. Pokrećite ga u [dry-run](#dry-run-mode) duže nego što biste pokrenuli predložak samo za čitanje, i stavite `edit_comment` iza [approval](#approval-workflow) dok ne budete imali povjerenja u procjenu modela za vaš saobraćaj.

### Okidači

- **Komentar izmenjen** (`COMMENT_EDIT`) - agent upoređuje novi i prethodni tekst i odlučuje da li izmjena izobličava odgovore koji već postoje.

Pogledajte [Trigger: Comment Edited](#trigger-comment-edit) za potpuni payload, uključujući prethodni tekst komentara i broj odgovora u trenutku izmjene.

### Dozvoljeni alati

- [`edit_comment`](#tool-edit-comment) - koristi se za vraćanje originalnog teksta kada se izmjena ocijeni kao gaslighting.
- [`warn_user`](#tool-warn-user) - izdaje blago upozorenje koje korisnik vidi pri sljedećoj posjeti.
- [`send_dm`](#tools-overview) - kanal za objašnjenje; korisnik dobija direktnu poruku koja opisuje zašto je njihova izmjena poništena.

Ne može banovati, označiti kao spam, glasati ili objavljivati nove komentare — domet je namjerno ograničen.

### Preporučena poboljšanja prije puštanja uživo

- **Stavite `edit_comment` iza [approval](#approval-workflow).** Vraćanje komentara je vidljivo autoru i svima koji su vidjeli izmenjenu verziju, tako da je lažno pozitivno neugodno. Držite odobrenja uključena dok dry-run ne pokaže da je agent konzistentan.
- **Sužite prompt sa primjerima šta na vašem sajtu predstavlja gaslighting.** Zadani prompt je kratak namjerno. Dajte modelu konkretne primjere — "promjena tvrdnje da/ ne", "brisanje broja na koji se odgovori pozivaju", "dodavanje neprijateljske rečenice nakon što su odgovori postavljeni" — i eksplicitne primjere šta nije gaslighting poput ispravki tipfelera, uređivanja formata ili dodavanja izvora.
- **Koristite broj odgovora iz konteksta okidača.** Izmjene komentara sa nula odgovora ne mogu izobličiti razgovor; prompt treba narediti modelu da preskoči takve izmjene.
- **Označite "Uključuje faktor povjerenja komentatora, starost naloga, istoriju banovanja i nedavne komentare"** u [Context Options](#context-options). Model je mnogo manje agresivan kada može vidjeti nalog koji dugo vremena radi u dobroj vjeri.
- **Razmotrite kratko vremensko ograničenje za izmjene u promptu.** Mnoge izmjene u prvih 30–60 sekundi su ispravke grešaka u kucanju; uputite model da ignoriše tako brze izmjene.

### Preporučeni period dry-run-a

Pokrenite najmanje dvije sedmice stvarnog saobraćaja u [dry-run](#dry-run-mode) prije nego što prebacite na Omogućeno, i pregledajte svaku označenu izmjenu tokom tog perioda. Koristite [Test Runs (Replays)](#test-runs-replays) da reprodukujete posljednjih 30 dana izmjena kroz agenta prije puštanja uživo.