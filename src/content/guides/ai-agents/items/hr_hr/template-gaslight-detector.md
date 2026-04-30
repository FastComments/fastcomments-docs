**ID predloška:** `gaslight_detector`

Gaslight Detector prati uređivanja komentara koja prepisuju povijest usred razgovora — onaj tip gdje autor promijeni značenje ranijeg komentara nakon što su odgovori već napisani, ostavljajući naknadne odgovore izvan konteksta ili pogrešnima. Kad agent prosudi da je uređivanje prešlo tu granicu, vraća izvorni tekst i šalje autoru privatnu poruku (DM) s objašnjenjem.

Ovo je predložak većeg rizika jer mijenja sadržaj korisnika. Pokrenite ga u [dry-run](#dry-run-mode) dulje nego što biste to učinili za predložak samo za čitanje, i stavite `edit_comment` iza [approval](#approval-workflow) dok ne budete sigurni u prosudbu modela za vaš promet.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - agent uspoređuje novi i prethodni tekst i odlučuje mijenja li uređivanje odgovore koji već postoje.

Pogledajte [Trigger: Comment Edited](#trigger-comment-edit) za puni payload, uključujući prethodni tekst komentara i broj odgovora u trenutku uređivanja.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - koristi se za vraćanje izvornog teksta kad se uređivanje proglasi gaslightingom.
- [`warn_user`](#tool-warn-user) - izdaje blago upozorenje koje korisnik vidi pri sljedećoj posjeti.
- [`send_dm`](#tools-overview) - kanal za objašnjenje; korisnik dobiva izravnu poruku s opisom zašto je njegovo uređivanje vraćeno.

Ne može zabranjivati, označavati kao spam, glasovati ili objavljivati nove komentare — površina je namjerno uska.

### Preporučeni dodaci prije puštanja u rad

- **Stavite `edit_comment` iza [approval](#approval-workflow).** Vraćanje komentara vidljivo je autoru i svima koji su vidjeli uređenu verziju, pa je lažno pozitivan rezultat neugodan. Držite odobrenja uključena dok dry-run ne pokaže da je agent dosljedan.
- **Pooštrite prompt s time što se na vašoj stranici smatra gaslightingom.** Zadani prompt je namjerno kratak. Dajte modelu konkretne primjere — "preokretanje tvrdnje da/ne", "brisanje broja na koji se odgovori pozivaju", "dodavanje neprijateljske rečenice nakon što su odgovori objavljeni" — i eksplicitne ne-primjere poput ispravaka tipfelera, čišćenja formata ili dodavanja izvora.
- **Koristite broj odgovora iz konteksta okidača.** Uređivanja komentara s nulom odgovora ne mogu iskriviti razgovor; prompt bi trebao reći modelu da preskoči takve slučajeve.
- **Označite "Uključi faktor povjerenja komentatora, starost računa, povijest zabrana i nedavne komentare"** u [Context Options](#context-options). Model je mnogo manje agresivan kada može vidjeti dugotrajni račun s dobronamjernom poviješću.
- **Razmotrite kratko razdoblje grace za uređivanje u promptu.** Mnoge izmjene u prvih 30–60 sekundi su ispravci tipfelera; uputite model da zanemari uređivanja koja su toliko brza.

### Preporučeno razdoblje dry-run-a

Pokrenite najmanje dva tjedna stvarnog prometa u [dry-run](#dry-run-mode) prije prebacivanja na Omogućeno i pregledajte svako označeno uređivanje tijekom tog razdoblja. Koristite [Test Runs (Replays)](#test-runs-replays) za ponovno reproduciranje zadnjih 30 dana uređivanja protiv agenta prije puštanja u rad.

---