**ID šablona:** `gaslight_detector`

Gaslight Detector prati izmene komentara kojima se prepravlja istorija usred razgovora - one gde autor menja značenje ranijeg komentara nakon što su odgovori već napisani, tako da naredni odgovori deluju van konteksta ili pogrešno. Kada agent odluči da izmena prelazi tu granicu, on vraća originalni tekst i šalje autoru DM sa objašnjenjem.

Ovo je šablon sa većim rizikom jer menja sadržaj korisnika. Pokrenite ga u [dry-run](#dry-run-mode) režimu duže nego što bi pokrenuli šablon koji samo čita, i ograničite `edit_comment` kroz [odobravanje](#approval-workflow) dok ne poverujete u modelovu procenu na vašem saobraćaju.

### Okidači

- **Komentar izmenjen** (`COMMENT_EDIT`) - agent poredi novi i prethodni tekst i odlučuje da li izmena narušava odgovore koji već postoje.

Pogledajte [Trigger: Comment Edited](#trigger-comment-edit) za kompletan payload, uključujući prethodni tekst komentara i broj odgovora u trenutku izmene.

### Dozvoljeni alati

- [`edit_comment`](#tool-edit-comment) - koristi se za vraćanje originalnog teksta kada se izmena proceni kao gaslighting.
- [`warn_user`](#tool-warn-user) - izdaje blago upozorenje koje korisnik vidi pri sledećoj poseti.
- [`send_dm`](#tools-overview) - kanal za objašnjenje; korisnik dobija direktnu poruku koja opisuje zašto je njegova izmena vraćena.

Ne može da banuje, označi spam, glasa niti objavljuje nove komentare - opseg je namerno uzak.

### Preporučene dopune pre prelaska u produkciju

- **Ograničite `edit_comment` kroz [odobravanje](#approval-workflow).** Vraćanje komentara je vidljivo autoru i svima koji su videli izmenjenu verziju, pa je lažni pozitivan rezultat sramota. Držite odobravanja uključenim dok dry-run ne pokaže da agent deluje dosledno.
- **Pojačajte prompt sa onim što se na vašem sajtu računa kao gaslighting.** Podrazumevani prompt je namerno kratak. Dajte modelu konkretne primere - "okretanje yes/no tvrdnje", "brisanje broja koji odgovori citiraju", "dodavanje neprijateljske rečenice nakon što su odgovori objavljeni" - i eksplicitne ne-primere poput ispravki kucnih grešaka, sređivanja formatiranja ili dodavanja izvora.
- **Koristite broj odgovora iz konteksta okidača.** Izmene komentara bez ijednog odgovora ne mogu narušiti razgovor; prompt treba da modelu kaže da ih preskoči.
- **Uključite "Include commenter's trust factor, account age, ban history, and recent comments"** u [Context Options](#context-options). Model je znatno manje agresivan kada vidi dugogodišnji nalog u dobrom verovanju.
- **Razmotrite kratak period milosti za izmene u promptu.** Mnoge izmene u prvih 30-60 sekundi su ispravke kucnih grešaka; uputite model da ignoriše tako brze izmene.

### Preporučeni period dry-run-a

Pokrenite najmanje dve nedelje stvarnog saobraćaja u [dry-run](#dry-run-mode) pre prelaska na Enabled, i pregledajte svaku označenu izmenu tokom tog perioda. Koristite [Test Runs (Replays)](#test-runs-replays) da ponovite poslednjih 30 dana izmena protiv agenta pre prelaska u produkciju.