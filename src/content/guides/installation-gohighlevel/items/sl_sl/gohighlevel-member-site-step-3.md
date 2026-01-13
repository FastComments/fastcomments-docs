Zdaj bomo ustvarili vašo prilagojeno kodo FastComments. Uporabite spodnji čarovnik, da konfigurirate, kako želite, da FastComments deluje na vašem GoHighLevel spletnem mestu:

[snippet id="gohighlevel-wizard"]

### Različne vrste polj za komentarje

Lahko konfigurirate vrstico `TYPE = 'commenting'`, da preklopite uporabljeni izdelek (na primer lahko spremenite v `live` za pretočni klepet ali v `collab` za collab klepet).

### Postavitev polja za komentarje tam, kjer želite

Recimo, da želite polja za komentarje postaviti v določene dele strani in ne na privzeta mesta.
Spremenite to vrstico:

    const TARGET_ELEMENT_ID = ''; // nastavite za uporabo načina ciljnega div elementa

V:

    const TARGET_ELEMENT_ID = 'fc_box'; // nastavite za uporabo načina ciljnega div elementa

Nato v urejevalniku GHL kliknite gumb "code" in dodajte mesto, kamor želite postaviti komentarje:

[inline-code-attrs-start title = 'GoHighLevel FastComments div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Različna vrsta polja za komentarje na posamezni strani

Recimo, da želite, da uporabniki označujejo in razpravljajo o delih besedila ali namesto tega uporabljajo vmesnik pretočnega klepeta.

Najprej sledite korakom zgoraj v razdelku "Postavitev polja za komentarje tam, kjer želite".

Upoštevajte, da je v tem kratkem odlomku `type="commenting"`.

Če želite na primer omogočiti collab klepet, spremenite type v `type="collab"`.

### Prikaži samo na določenih straneh

Če ne nastavite `TARGET_ELEMENT_ID`, lahko namesto tega nastavite spremenljivko `VALID_PATTERNS`, da določite, na katerih URL poteh se bodo komentarji prikazovali. Privzeto se prikažejo
na straneh, ki v URL vsebujejo `/post`.

### Konfiguriranje collab klepeta

Lahko poveste collab klepetu, naj doda sodelovalno funkcionalnost le okoli HTML v določenem območju, na primer, recimo, da dodate zgornjo kodo v nogo in nato v vsebino objave/strani vstavite ta div, da omogočite collab klepet:

[inline-code-attrs-start title = 'Collab klepet z določenimi vsebinami'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Nato bo element <p> znotraj `<div>` imel omogočen collab klepet, in nič drugega na strani. Če v `<div>` ne vstavite nobene vsebine, bo collab klepet omogočen za celotno telo objave.