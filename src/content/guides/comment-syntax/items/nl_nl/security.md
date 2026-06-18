Er zijn meerdere aspecten aan beveiliging wanneer je mensen content laat toevoegen aan een website
en die content vervolgens op veel verschillende soorten apparaten weergeeft.

### Voorkomen van misbruik van opmaak

Mensen kunnen inhoud schrijven die opzettelijk visueel afleidt
en de waarde van discussies vermindert door misbruik te maken van tekstopmaak.

FastComments doet een aantal dingen om misbruik met betrekking tot opmaak te voorkomen:

- Grote herhaalde opeenvolgende lege regels worden samengevoegd.
- We tonen geen koppen (ze worden normale tekst).
- We staan geen CSS of aangepaste kleuren toe.

### Voorkomen van exploits

In systemen die HTML renderen kunnen exploits ontstaan. FastComments doet een aantal dingen om dit te voorkomen:

- We staan alleen een expliciet gedefinieerde set HTML-tags toe.
- We staan alleen een expliciet gedefinieerde set HTML-tagattributen toe.
- We zuiveren en saneren alle invoer.
  - Dit gebeurt via de [DOMPurify](https://www.npmjs.com/package/dompurify) en [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) bibliotheken.
  - We hebben deze bibliotheken gekozen omdat ze goed getest zijn (met respectievelijk meer dan 4 en 1 miljoen downloads per week).

Dit betekent dat gebruikers niet dingen kunnen doen zoals het schrijven van `<script>` of `<style>` tags, of proberen `onload=alert()`-achtige scripts toe te voegen aan afbeeldingen of andere content.

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

De `<iframe>`-tag is standaard niet toegestaan. Als je 'Media-embeds toestaan' inschakelt, zijn iframes ook toegestaan, maar alleen wanneer hun bron één is van een ingebouwde lijst met vertrouwde providers (zoals YouTube, Vimeo, SoundCloud en Spotify) of een hostnaam die je expliciet hebt toegevoegd. Iframes van andere bronnen worden verwijderd.