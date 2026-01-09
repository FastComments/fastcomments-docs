Sada ćemo generirati vaš prilagođeni FastComments kod. Upotrijebite čarobnjak dolje za konfiguriranje kako želite da FastComments radi na vašoj GoHighLevel stranici:

[snippet id="gohighlevel-wizard"]

### Različite vrste okvira za komentare

Možete konfigurirati liniju `TYPE = 'commenting'` da promijenite proizvod koji se koristi (na primjer možete ga promijeniti u `live` za streaming chat ili `collab` za kolaboracijski chat).

### Postavljanje okvira za komentare gdje želite

Recimo da želite staviti okvire za komentare na određene dijelove stranice, a ne na zadane lokacije.
Promijenite ovu liniju:

    const TARGET_ELEMENT_ID = ''; // postavi da koristi target div način

U:

    const TARGET_ELEMENT_ID = 'fc_box'; // postavi da koristi target div način

Zatim u GHL uređivaču kliknite gumb "code" i dodajte mjesto gdje želite da komentari budu:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Različita vrsta okvira za komentare po stranici

Recimo da želite da korisnici označavaju i raspravljaju dijelove teksta, ili koriste sučelje za streaming chat umjesto toga.

Prvo slijedite korake gore u "Postavljanje okvira za komentare gdje želite".

Obratite pažnju u tom malom isječku postoji `type="commenting"`.

Ako želite omogućiti collab chat na primjer promijenite type u `type="collab"`.

### Prikaz samo na određenim stranicama

Ako ne postavite ne postavite `TARGET_ELEMENT_ID`, umjesto toga možete konfigurirati varijablu `VALID_PATTERNS`, za postavljanje na kojim URL rutama bi se komentari trebali prikazivati. Po zadanom, prikazivat će se
na stranicama koje u URL-u sadrže `/post`.

### Konfiguriranje Collab Chata

Možete reći collab chatu da doda kolaboracijsku funkcionalnost samo oko HTML-a unutar određenog područja, na primjer, recimo da
dodate gore navedeni footer kod a zatim dodate ovaj div u sadržaj posta/stranice da omogućite collab chat:

[inline-code-attrs-start title = 'Collab chat s određenim sadržajem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Tada će element paragraf unutar `<div>` imati omogućen collab chat, i ništa drugo na stranici. Ako ne
stavite nikakav sadržaj u `<div>` tada će se omogućiti collab chat na cijelom tijelu posta.