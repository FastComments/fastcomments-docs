Postoji više aspekata bezbednosti kada ljudima dozvoljavamo dodavanje sadržaja na veb stranicu i zatim prikazivanje tog sadržaja na mnogim različitim vrstama uređaja.

### Sprečavanje zloupotrebe formatiranja

Ljudi mogu pisati sadržaj koji je namerno vizuelno ometajući i umanjuje vrednost diskusija zloupotrebom formatiranja teksta.

FastComments čini nekoliko stvari da bi sprečio zloupotrebu u vezi sa formatiranjem:

- Veliki ponovljeni uzastopni prelomi redova se sažimaju.
- Ne prikazujemo naslove (postaju običan tekst).
- Ne dozvoljavamo CSS ili prilagođene boje.

### Sprečavanje eksploita

Eksploiti se mogu stvoriti u sistemima koji prikazuju HTML. FastComments čini nekoliko stvari da bi to sprečio:

- Dozvoljavamo samo eksplicitno definisan skup HTML oznaka.
- Dozvoljavamo samo eksplicitno definisan skup atributa HTML oznaka.
- Prečišćavamo i sanitizujemo sve unose.
  - To se radi putem [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) biblioteka.
  - Odabrali smo ove biblioteke jer su dobro testirane (sa više od 4 i 1 milion preuzimanja nedeljno, respektivno).

To znači da korisnici ne mogu raditi stvari poput pisanja `<script>` ili `<style>` oznaka, ili pokušavati dodati `onload=alert()` tipa skripte slikama ili drugom sadržaju.

HTML oznake koje dozvoljavamo su sledeće:

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
