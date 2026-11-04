Postoji više aspekata bezbednosti kada se korisnicima dozvoljava dodavanje sadržaja na veb-sajt,
a zatim taj sadržaj prikazuje na mnogim različitim vrstama uređaja.

### Sprečavanje zloupotrebe formatiranja

Korisnici mogu pisati sadržaj koji je namerno vizuelno ometajući
i umanjuje vrednost diskusija zloupotrebom formatiranja teksta.

FastComments čini niz stvari kako bi sprečio zloupotrebu u vezi sa formatiranjem:

- Veliki broj uzastopnih praznih redova se sažima.
- Ne prikazujemo naslove (oni postaju običan tekst).
- Ne dozvoljavamo CSS ni prilagođene boje.

### Sprečavanje eksploatacija (exploits)

Eksploatacije mogu biti kreirane u sistemima koji prikazuju HTML. FastComments čini nekoliko stvari kako bi ovo sprečio:

- Dozvoljavamo samo eksplicitno definisan skup HTML tagova.
- Dozvoljavamo samo eksplicitno definisan skup atributa HTML tagova.
- Pročišćavamo i sanitizujemo sve unose.
  - Ovo se radi putem biblioteka [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Odabrali smo ove biblioteke jer su dobro testirane (sa preko 4 odnosno 1 milion preuzimanja nedeljno).

To znači da korisnici ne mogu da rade stvari poput pisanja `<script>` ili `<style>` tagova, niti da pokušaju da dodaju skripte tipa `onload=alert()` na slike ili drugi sadržaj.

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

Tag `<iframe>` podrazumevano nije dozvoljen. Ako uključite opciju Allow Media Embeds, iframes su takođe dozvoljeni, ali samo kada je njihov izvor jedan od ugrađenog spiska pouzdanih provajdera (kao što su YouTube, Vimeo, SoundCloud i Spotify) ili hostname koji ste eksplicitno dodali. Iframes iz bilo kog drugog izvora se uklanjaju.