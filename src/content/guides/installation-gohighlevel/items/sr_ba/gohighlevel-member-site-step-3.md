Sada ćemo generisati vaš prilagođeni FastComments kod. Koristite čarobnjak ispod da konfigurirate kako želite da FastComments radi na vašem GoHighLevel sajtu:

[snippet id="gohighlevel-wizard"]

### Različite vrste kutija za komentare

Možete konfigurirati liniju `TYPE = 'commenting'` da zamijenite proizvod koji se koristi (na primjer možete promijeniti u `live` za streaming chat ili `collab` za collab chat).

### Postavljanje kutije za komentare tamo gdje želite

Pretpostavimo da želite staviti kutije za komentare na određene dijelove stranice, a ne na podrazumijevane lokacije.
Promijenite ovaj red:

    const TARGET_ELEMENT_ID = ''; // postavite da koristi režim target div

U:

    const TARGET_ELEMENT_ID = 'fc_box'; // postavite da koristi režim target div

Zatim u GHL editoru, kliknite dugme "code" i dodajte gdje želite da idu komentari:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Različita vrsta kutije za komentare po stranici

Recimo da želite da korisnici označavaju i raspravljaju dijelove teksta, ili da umjesto toga koriste UI za streaming chat.

Prvo slijedite korake gore u "Postavljanje kutije za komentare tamo gdje želite".

Primijetite u tom malom isječku postoji `type="commenting"`.

Ako želite omogućiti collab chat, na primjer promijenite type u `type="collab"`.

### Prikazivati samo na određenim stranicama

Ako ne postavite ne postavite `TARGET_ELEMENT_ID`, možete umjesto toga konfigurirati varijablu `VALID_PATTERNS`, da podesite na kojim URL rutama bi se komentari trebali prikazivati. Po defaultu, prikazivat će se
na stranicama koje u URL-u sadrže `/post`.

### Konfiguriranje Collab Chata

Možete reći collab chatu da doda kolaborativnu funkcionalnost samo oko HTML-a unutar određenog područja, na primjer, recimo da
dodate footer kod gore, a zatim dodate ovaj div u sadržaj posta/stranice da omogućite collab chat:

[inline-code-attrs-start title = 'Collab chat sa određenim sadržajem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Tada će paragraf element unutar `<div>` imati omogućen collab chat, i ništa drugo na stranici. Ako ne stavite nikakav sadržaj u `<div>` onda će omogućiti collab chat na cijelom tijelu posta.

---