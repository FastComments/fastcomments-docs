Omogočanje uporabnikom dodajanja vsebine na spletno stran in nato prikaz te vsebine na številnih različnih vrstah naprav prinaša več vidikov varnosti.

### Preprečevanje zlorabe oblikovanja

Ljudje lahko pišejo vsebino, ki je namenoma vizualno moteča in odvreda vrednost razprav, z zlorabo oblikovanja besedila.

FastComments naredi več stvari za preprečevanje zlorabe v zvezi z oblikovanjem:

- Velike ponavljajoče se zaporedne prelome vrstic se strnejo.
- Naslovov ne prikazujemo (postanejo običajno besedilo).
- CSS ali barv po meri ne dovoljujemo.

### Preprečevanje izkoriščanja ranljivosti

Izkoriščanja ranljivosti lahko nastanejo v sistemih, ki prikazujejo HTML. FastComments naredi več stvari za preprečevanje tega:

- Dovoljujemo samo izrecno določen nabor oznak HTML.
- Dovoljujemo samo izrecno določen nabor atributov oznak HTML.
- Vse vnose očistimo in sanitiziramo.
  - To naredimo s knjižnicama [DOMPurify](https://www.npmjs.com/package/dompurify) in [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Izbrali smo ti knjižnici, ker sta dobro preizkušeni (vsaka ima več kot 4 oziroma 1 milijon prenosov na teden).

To pomeni, da uporabniki ne morejo početi stvari, kot je pisanje oznak `<script>` ali `<style>`, ali poskus dodajanja skript vrste `onload=alert()` slikam ali drugi vsebini.

Oznake HTML, ki jih dovoljujemo, so naslednje:

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

Oznaka `<iframe>` privzeto ni dovoljena. Če vklopite možnost Allow Media Embeds, so iframi prav tako dovoljeni, vendar le, kadar je njihov vir en od vgrajenega seznama zaupanja vrednih ponudnikov (kot so YouTube, Vimeo, SoundCloud in Spotify) ali gostiteljsko ime, ki ste ga izrecno dodali. Iframi iz katerega koli drugega vira so odstranjeni.