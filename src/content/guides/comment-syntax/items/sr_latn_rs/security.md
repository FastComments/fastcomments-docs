Postoji više aspekata bezbednosti kada ljudima dozvolite da dodaju sadržaj na veb-sajt i potom prikažete taj sadržaj na mnogim različitim tipovima uređaja.

### Sprečavanje zloupotrebe formatiranja

Ljudi mogu pisati sadržaj koji je namerno vizuelno ometajući i umanjuje vrednost diskusija zloupotrebom formatiranja teksta.

FastComments preduzima niz mera da spreči zloupotrebe vezane za formatiranje:

- Veliki uzastopni prazni redovi se skraćuju.
- Ne prikazujemo naslove (oni postaju običan tekst).
- Ne dozvoljavamo CSS ili prilagođene boje.

### Sprečavanje eksploatacija

Eksploatacije se mogu stvoriti u sistemima koji renderuju HTML. FastComments preduzima nekoliko mera da to spreči:

- Dozvoljavamo samo eksplicitno definisan skup HTML tagova.
- Dozvoljavamo samo eksplicitno definisan skup atributa HTML tagova.
- Pročišćavamo i sanitizujemo sve ulaze.
  - Ovo se radi putem biblioteka [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Izabrali smo ove biblioteke jer su dobro testirane (imaju više od 4 i 1 milion preuzimanja nedeljno, respektivno).

To znači da korisnici ne mogu, na primer, pisati `<script>` ili `<style>` tagove, niti pokušavati da dodaju skripte tipa `onload=alert()` na slike ili drugi sadržaj.

HTML tagovi koje dozvoljavamo su sledeći:

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

The `<iframe>` tag is not allowed by default. If you turn on Allow Media Embeds, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.