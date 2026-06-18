---
Der er flere aspekter af sikkerhed, når man lader folk tilføje indhold til en hjemmeside
og så gengive det indhold på mange forskellige typer enheder.

### Forebyggelse af misbrug af formatering

Folk kan skrive indhold, der bevidst er visuelt forstyrrende
og reducerer værdien af diskussioner ved at misbruge tekstformatering.

FastComments gør flere ting for at forhindre misbrug i forbindelse med formatering:

- Store gentagne sammenhængende linjeskift bliver slået sammen.
- Vi gengiver ikke overskrifter (de bliver almindelig tekst).
- Vi tillader ikke CSS eller brugerdefinerede farver.

### Forebyggelse af udnyttelser

Udnyttelser kan opstå i systemer, der gengiver HTML. FastComments gør flere ting for at forhindre dette:

- Vi tillader kun et eksplicit defineret sæt af HTML-tags.
- Vi tillader kun et eksplicit defineret sæt af HTML-tag-attributter.
- Vi rensker og sanerer alle input.
  - Dette gøres via [DOMPurify](https://www.npmjs.com/package/dompurify) og [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) bibliotekerne.
  - Vi valgte disse biblioteker, da de er velafprøvede (med over 4 millioner og 1 million downloads pr. uge, henholdsvis).

Det betyder, at brugere ikke kan gøre ting som at skrive `<script>` eller `<style>` tags, eller forsøge at tilføje `onload=alert()`-type scripts til billeder eller andet indhold.

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

The `<iframe>` tag is not allowed by default. If you turn on Allow Media Embeds, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.

---