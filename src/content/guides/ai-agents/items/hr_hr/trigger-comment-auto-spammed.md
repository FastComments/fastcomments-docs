Javlja se kada je komentar automatski označen kao spam od FastComments-ovog ugrađenog spam mehanizma - **ne** od strane moderatora i ne od strane drugog agenta.

### Kontekst koji agent prima

- Komentar automatski označen kao spam.
- Opcionalna povijest razgovora / korisnika / kontekst stranice kako je konfigurirano.

### Tko pokreće ovo

Spam pipeline platforme. Pogledajte [Spam Detection](/guide-moderation.html#spam-detection) u vodiču za moderiranje za više detalja.

### Uobičajene upotrebe

- **Drugi pregled moderacije** - spam mehanizam ima visoku odzivnost ali nedovoljnu preciznost; agent obučen za stil vaše zajednice može uhvatiti lažno pozitivne. Agent može pozvati funkciju za uklanjanje oznake sa pogrešno klasificiranog komentara.
- **Automatsko ukidanje zabrane** - ako vaš tenant agresivno spam-zabranjuje nove račune, agent na ovom okidaču može pregledati i očistiti očite lažno pozitivne prije nego što ih čovjek uopće vidi.

### Napomena

- Okidač se **ne** aktivira za spam označen od strane moderatora (use [Trigger: Moderator Marked Spam](#trigger-moderator-spammed)) niti za spam označen od strane drugog agenta.
- Komentar koji je automatski označen kao spam, a zatim kasnije označen kao Not Spam od strane moderatora, ponovno ne aktivira ovaj okidač.
- Pretplata na ovaj okidač najkorisnija je u tenantima gdje je automatski način rada spam motora omogućen u Moderation Settings. Inače se okidač neće aktivirati.