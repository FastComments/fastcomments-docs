Postoji više aspekata sigurnosti kada ljudima dopuštamo dodavanje sadržaja na web stranicu i zatim prikazivanje tog sadržaja na mnogim različitim vrstama uređaja.

### Sprječavanje zlouporabe formatiranja

Ljudi mogu pisati sadržaj koji je namjerno vizualno ometajući i umanjuje vrijednost rasprava zloupotrebom formatiranja teksta.

FastComments čini nekoliko stvari kako bi spriječio zlouporabu u vezi s formatiranjem:

- Veliki ponovljeni uzastopni prijelomi redaka se sažimaju.
- Ne prikazujemo naslove (postaju obični tekst).
- Ne dopuštamo CSS ili prilagođene boje.

### Sprječavanje exploita

Exploiti se mogu stvoriti u sustavima koji prikazuju HTML. FastComments čini nekoliko stvari kako bi to spriječio:

- Dopuštamo samo eksplicitno definirani skup HTML oznaka.
- Dopuštamo samo eksplicitno definirani skup atributa HTML oznaka.
- Pročišćavamo i sanitiziramo sve unose.
  - To se radi putem [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) biblioteka.
  - Odabrali smo ove biblioteke jer su dobro testirane (s više od 4 i 1 milijun preuzimanja tjedno, respektivno).

To znači da korisnici ne mogu raditi stvari poput pisanja `<script>` ili `<style>` oznaka, ili pokušavati dodati `onload=alert()` tipa skripte slikama ili drugom sadržaju.

HTML oznake koje dopuštamo su sljedeće:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`
