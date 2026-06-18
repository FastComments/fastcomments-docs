Postoji više aspekata sigurnosti kada dopustite ljudima da dodaju sadržaj na web-stranicu
i onda taj sadržaj prikazujete na mnogim različitim vrstama uređaja.

### Sprječavanje zlouporabe formatiranja

Ljudi mogu pisati sadržaj koji je namjerno vizualno ometajući
i umanjuje vrijednost rasprava zloupotrebom oblikovanja teksta.

FastComments poduzima niz mjera kako bi spriječio zloupotrebu u vezi s formatiranjem:

- Velike ponovljene uzastopne prazne retke se sažimaju.
- Naslove ne prikazujemo kao naslove (postaju običan tekst).
- Ne dopuštamo CSS ili prilagođene boje.

### Sprječavanje iskorištavanja

U sustavima koji prikazuju HTML mogu nastati eksploiti. FastComments poduzima nekoliko mjera kako bi to spriječio:

- Dopuštamo samo eksplicitno definiran skup HTML oznaka.
- Dopuštamo samo eksplicitno definiran skup atributa HTML oznaka.
- Pročišćavamo i sanitiziramo sve unose.
  - Ovo se radi pomoću biblioteka [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Odabrali smo ove biblioteke jer su dobro testirane (imaju preko 4 i 1 milijuna preuzimanja tjedno, redom).

To znači da korisnici ne mogu raditi stvari poput pisanja `<script>` ili `<style>` oznaka, ili pokušavati dodati skripte tipa `onload=alert()` slikama ili drugom sadržaju.

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

The `<iframe>` tag is not allowed by default. Ako omogućite opciju Omogući ugrađivanje medija, iframeovi su također dopušteni, ali samo kada im je izvor jedna od ugrađenih stavki na popisu pouzdanih pružatelja (kao što su YouTube, Vimeo, SoundCloud, i Spotify) ili hostname koji ste eksplicitno dodali. Iframeovi iz bilo kojeg drugog izvora bit će uklonjeni.