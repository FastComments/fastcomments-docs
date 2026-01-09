Sada ćemo generisati vaš prilagođeni FastComments kod. Koristite čarobnjak ispod da konfigurišete kako želite da FastComments radi na vašem GoHighLevel sajtu:

[snippet id="gohighlevel-wizard"]

### Različite vrste polja za komentare

Možete podesiti liniju `TYPE = 'commenting'` da promenite koji se proizvod koristi (na primer, možete je promeniti u `live` za streaming chat ili `collab` za collab chat).

### Postavljanje polja za komentare tamo gde želite

Recimo da želite postaviti polja za komentare na određene delove stranice, a ne na podrazumevane lokacije.
Promenite ovu liniju:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

U:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Zatim u GHL editoru kliknite na dugme "code" i dodajte gde želite da komentari budu prikazani:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Različita vrsta polja za komentare po stranici

Recimo da želite da korisnici označavaju i diskutuju delove teksta, ili da umesto toga koriste streaming chat UI.

Prvo sledite korake iz odeljka "Postavljanje polja za komentare tamo gde želite".

Primetite da u tom malom snippetu stoji `type="commenting"`.

Ako želite, na primer, da omogućite collab chat, promenite type u `type="collab"`.

### Prikazivati samo na određenim stranicama

Ako ne podesite `TARGET_ELEMENT_ID`, umesto toga možete konfigurisati promenljivu `VALID_PATTERNS`, da odredite na kojim URL rutama treba da se prikazuju komentari. Po podrazumevanju prikazivaće se na stranicama koje u URL-u sadrže `/post`.

### Konfigurisanje Collab chata

Možete reći collab chatu da doda kolaborativnu funkcionalnost samo oko HTML-a unutar određenog područja; na primer, recimo da dodate footer kod iznad, a zatim ubacite ovaj div u sadržaj posta/stranice da biste omogućili collab chat:

[inline-code-attrs-start title = 'Collab chat sa određenim sadržajem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Tada će paragraf unutar `<div>` elementa imati omogućen collab chat, a ništa drugo na stranici neće. Ako ne stavite nikakav sadržaj u `<div>` tada će collab chat biti omogućen na celom sadržaju posta.

---