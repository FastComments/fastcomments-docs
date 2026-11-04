Der er flere aspekter af sikkerhed, når man lader folk tilføje indhold til en hjemmeside
og derefter renderer det indhold på mange forskellige typer enheder.

### Forebyggelse af misbrug af formatering

Folk kan skrive indhold, der bevidst er visuelt forstyrrende
og reducerer værdien af diskussioner ved at misbruge tekstformatering.

FastComments gør en række ting for at forebygge misbrug i forhold til formatering:

- Store gentagne sekvenser af linjeskift sammenlægges.
- Vi renderer ikke overskrifter (de bliver til normal tekst).
- Vi tillader ikke CSS eller brugerdefinerede farver.

### Forebyggelse af exploits

Exploits kan oprettes i systemer, der renderer HTML. FastComments gør flere ting for at forebygge dette:

- Vi tillader kun et eksplicit defineret sæt af HTML-tags.
- Vi tillader kun et eksplicit defineret sæt af HTML-tag-attributter.
- Vi renser og saniterer alle input.
  - Dette gøres via bibliotekerne [DOMPurify](https://www.npmjs.com/package/dompurify) og [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Vi valgte disse biblioteker, fordi de er velafprøvede (med henholdsvis over 4 og 1 millioner downloads pr. uge).

Det betyder, at brugere ikke kan gøre ting som at skrive `<script>`- eller `<style>`-tags, eller forsøge at tilføje `onload=alert()`-type scripts til billeder eller andet indhold.

De HTML-tags, vi tillader, er som følger:

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

Tagget `<iframe>` er ikke tilladt som standard. Hvis du aktiverer Allow Media Embeds, tillades iframes også, men kun når deres kilde er en af en indbygget liste over betroede udbydere (såsom YouTube, Vimeo, SoundCloud og Spotify) eller et hostname, du eksplicit har tilføjet. Iframes fra enhver anden kilde fjernes.