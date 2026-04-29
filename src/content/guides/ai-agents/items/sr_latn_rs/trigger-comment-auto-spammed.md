---
Pokreće se kada je komentar automatski označen kao spam od strane ugrađenog spam motora FastComments — **ne** od strane moderatora i ne od drugog agenta.

### Kontekst koji agent prima

- Komentar koji je automatski označen kao spam.
- Opcionalna istorija teme / korisnika / kontekst stranice kako je konfigurisan.

### Ko pokreće ovo

Spam pipeline platforme. Pogledajte [Otkrivanje spama](/guide-moderation.html#spam-detection) u vodiču za moderaciju za više detalja.

### Uobičajene upotrebe

- **Druga provera moderacije** - spam motor ima visok recall ali ne i savršenu preciznost; agent obučen na specifičan stil vaše zajednice može uhvatiti lažno pozitivne slučajeve. Agent može izvršiti poziv da ukloni oznaku sa pogrešno klasifikovanog komentara.
- **Automatsko uklanjanje zabrane** - ako vaš tenant agresivno spam-zabranjuje nove naloge, agent na ovom okidaču može pregledati i ukloniti očigledne lažno pozitivne slučajeve pre nego što ih ljudski moderator ikada vidi.

### Napomena

- Okidač se **ne** pokreće za spam označen od strane moderatora (koristite [Okidač: Moderatorom označen spam](#trigger-moderator-spammed)) niti za spam označen od strane drugog agenta.
- Komentar koji je automatski označen kao spam, a kasnije moderator označi kao 'Nije spam', neće ponovo pokrenuti okidač.
- Pretplata na ovaj okidač je najkorisnija u tenantima gde je režim automatskog spamovanja motora omogućen u Podešavanjima moderacije. U suprotnom okidač se neće pokrenuti.

---