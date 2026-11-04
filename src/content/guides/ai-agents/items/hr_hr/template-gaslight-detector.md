**ID predloška:** `gaslight_detector`

Gaslight Detector prati uređivanja komentara koja prepisuju povijest usred razgovora - onu vrstu gdje autor mijenja značenje ranijeg komentara nakon što su odgovori već napisani, tako da kasniji odgovori djeluju izvan konteksta ili pogrešno. Kada agent odluči da uređivanje prelazi tu granicu, vraća izvorni tekst i šalje autoru izravnu poruku s objašnjenjem.

Ovo je rizičniji predložak jer mijenja korisnički sadržaj. Pokrenite ga u [dry-run](#dry-run-mode) načinu dulje nego što biste pokrenuli predložak koji samo čita, i držite `edit_comment` iza [odobrenja](#approval-workflow) dok ne povjerujete u modelovu prosudbu na vašem prometu.

### Okidači

- **Komentar uređen** (`COMMENT_EDIT`) - agent uspoređuje novi i prethodni tekst te odlučuje mijenja li uređivanje smislenost odgovora koji već postoje.

Pogledajte [Trigger: Comment Edited](#trigger-comment-edit) za potpuni sadržaj, uključujući prethodni tekst komentara i broj odgovora u trenutku uređivanja.

### Dopušteni alati

- [`edit_comment`](#tool-edit-comment) - koristi se za vraćanje izvornog teksta kada se uređivanje ocijeni kao gaslighting.
- [`warn_user`](#tool-warn-user) - izdaje blago upozorenje koje korisnik vidi pri sljedećoj posjeti.
- [`send_dm`](#tools-overview) - kanal za objašnjenje; korisnik prima izravnu poruku koja opisuje zašto je njegovo uređivanje poništeno.

Ne može banirati, označiti kao spam, glasati ni objavljivati nove komentare - područje je namjerno usko.

### Preporučene dopune prije objave

- **Držite `edit_comment` iza [odobrenja](#approval-workflow).** Poništavanje komentara vidljivo je autoru i svima koji su vidjeli uređenu verziju, pa je lažni pozitivan rezultat neugodan. Držite odobrenja uključenima dok dry-run ne pokaže da je agent dosljedan.
- **Pojačajte prompt s time što se na vašoj stranici smatra gaslightingom.** Zadani prompt je namjerno kratak. Dajte modelu konkretne primjere - "okretanje da/ne tvrdnje", "brisanje broja koji odgovori citiraju", "dodavanje neprijateljske rečenice nakon što su odgovori objavljeni" - i eksplicitne ne-primjere poput ispravaka tipfelera, čišćenja formatiranja ili dodavanja izvora.
- **Koristite broj odgovora iz konteksta okidača.** Uređivanja komentara bez odgovora ne mogu iskriviti razgovor; prompt treba reći modelu da ih preskoči.
- **Uključite "Include commenter's trust factor, account age, ban history, and recent comments"** u [Context Options](#context-options). Model je znatno manje agresivan kada vidi dugogodišnji račun s dobrom namjerom.
- **Razmotrite kratko razdoblje dopuštenog uređivanja u promptu.** Mnoga uređivanja u prvim 30-60 sekundi su ispravci tipfelera; recite modelu da ignorira tako brza uređivanja.

### Preporučeno razdoblje dry-run načina

Pokrenite najmanje dva tjedna stvarnog prometa u [dry-run](#dry-run-mode) načinu prije prebacivanja na Enabled, i pregledajte svako označeno uređivanje tijekom tog razdoblja. Koristite [Test Runs (Replays)](#test-runs-replays) da ponovite zadnjih 30 dana uređivanja naspram agenta prije objave.