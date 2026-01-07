Er zijn meerdere aspecten van beveiliging wanneer je mensen toestaat inhoud toe te voegen aan een website en die inhoud vervolgens weer te geven op veel verschillende soorten apparaten.

### Voorkomen van opmaakmisbruik

Mensen kunnen inhoud schrijven die opzettelijk visueel afleidend is en de waarde van discussies vermindert door misbruik te maken van tekstopmaak.

FastComments doet een aantal dingen om misbruik met betrekking tot opmaak te voorkomen:

- Grote herhaalde opeenvolgende regeleinden worden samengevouwen.
- We renderen geen koppen (ze worden normale tekst).
- We staan geen CSS of aangepaste kleuren toe.

### Voorkomen van exploits

Exploits kunnen worden gemaakt in systemen die HTML renderen. FastComments doet verschillende dingen om dit te voorkomen:

- We staan alleen een expliciet gedefinieerde set HTML-tags toe.
- We staan alleen een expliciet gedefinieerde set HTML-tag-attributen toe.
- We zuiveren en saneren alle invoer.
  - Dit wordt gedaan via de [DOMPurify](https://www.npmjs.com/package/dompurify) en [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) bibliotheken.
  - We kozen deze bibliotheken omdat ze goed getest zijn (met meer dan 4 en 1 miljoen downloads per week, respectievelijk).

Dit betekent dat gebruikers geen dingen kunnen doen zoals `<script>`- of `<style>`-tags schrijven, of proberen `onload=alert()`-type scripts toe te voegen aan afbeeldingen of andere inhoud.

De HTML-tags die we toestaan zijn als volgt:

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
