Der er flere aspekter af sikkerhed, når man lader folk tilføje indhold til en hjemmeside og derefter viser dette indhold på mange forskellige typer enheder.

### Forebyggelse af formateringsmisbrug

Folk kan skrive indhold, der bevidst er visuelt distraherende og forringer værdien af diskussioner ved at misbruge tekstformatering.

FastComments gør en række ting for at forhindre misbrug med hensyn til formatering:

- Store gentagne på hinanden følgende linjeskift kollapses.
- Vi renderer ikke overskrifter (de bliver til normal tekst).
- Vi tillader ikke CSS eller brugerdefinerede farver.

### Forebyggelse af exploits

Exploits kan skabes i systemer, der renderer HTML. FastComments gør flere ting for at forhindre dette:

- Vi tillader kun et eksplicit defineret sæt af HTML-tags.
- Vi tillader kun et eksplicit defineret sæt af HTML-tag-attributter.
- Vi renser og saniterer alle inputs.
  - Dette gøres via [DOMPurify](https://www.npmjs.com/package/dompurify) og [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) bibliotekerne.
  - Vi valgte disse biblioteker som værende veltestede (med over 4 og 1 million downloads pr. uge, henholdsvis).

Dette betyder, at brugere ikke kan gøre ting som at skrive `<script>` eller `<style>` tags, eller forsøge at tilføje `onload=alert()` type scripts til billeder eller andet indhold.

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
