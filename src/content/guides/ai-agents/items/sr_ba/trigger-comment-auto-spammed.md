---
Okida se kada komentar automatski označi kao spam od strane ugrađenog spam motora FastComments-a - **ne** od moderatora i ne od drugog agenta.

### Kontekst koji agent prima

- Komentar koji je automatski označen kao spam.
- Neobavezna historija teme / korisnika / kontekst stranice, kako je konfigurisano.

### Ko pokreće ovaj okidač

Spam pipeline platforme. Pogledajte [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju za više detalja.

### Uobičajene upotrebe

- **Ponovna provjera moderacije** - spam motor ima visok odziv (recall) ali nedovoljnu preciznost; agent obučen za specifičan stil vaše zajednice može uhvatiti lažno pozitivne slučajeve. Agent može ukloniti oznaku sa pogrešno klasifikovanog komentara.
- **Automatsko uklanjanje zabrane** - ako vaš tenant agresivno blokira nove naloge zbog spama, agent aktiviran ovim okidačem može pregledati i ukloniti očite lažne pozitivne prije nego što ih čovjek uopće vidi.

### Napomena

- Okidač se **ne** pokreće za spam koji je moderator označio (koristite [Okidač: Moderatorom označen spam](#trigger-moderator-spammed)) niti za spam koji je označio drugi agent.
- Komentar koji je automatski označen kao spam, a koji moderator kasnije označi kao 'Not Spam', ne pokreće ponovo ovaj okidač.
- Pretplata na ovaj okidač najkorisnija je u tenantima gdje je automatski režim spama motora omogućen u Postavkama moderacije. U suprotnom, okidač se neće aktivirati.

---