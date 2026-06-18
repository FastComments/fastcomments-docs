Ci sono diversi aspetti della sicurezza quando si permette alle persone di aggiungere contenuti a un sito web e poi rendere quei contenuti visibili su molti tipi di dispositivi.

### Prevenire l'abuso del formato

Le persone possono scrivere contenuti intenzionalmente visivamente fastidiosi e che riducono il valore delle discussioni abusando della formattazione del testo.

FastComments fa diverse cose per prevenire abusi riguardo alla formattazione:

- Le grandi interruzioni di riga consecutive ripetute vengono collassate.
- Non rendiamo le intestazioni (diventano testo normale).
- Non permettiamo CSS o colori personalizzati.

### Prevenire gli exploit

Gli exploit possono essere creati nei sistemi che renderizzano HTML. FastComments fa diverse cose per prevenire questo:

- Permettiamo solo un insieme esplicitamente definito di tag HTML.
- Permettiamo solo un insieme esplicitamente definito di attributi dei tag HTML.
- Purifichiamo e sanifichiamo tutti gli input.
  - Questo è fatto tramite le librerie [DOMPurify](https://www.npmjs.com/package/dompurify) e [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Abbiamo scelto queste librerie perché sono ben testate (avendo rispettivamente oltre 4 e 1 milioni di download alla settimana).

Questo significa che gli utenti non possono fare cose come scrivere `<script>` o `<style>` tag, o provare ad aggiungere script tipo `onload=alert()` alle immagini o ad altri contenuti.

I tag HTML che permettiamo sono i seguenti:

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

Il tag `<iframe>` non è consentito di default. Se abiliti Allow Media Embeds, anche gli iframe sono permessi, ma solo quando la loro sorgente è uno degli elenchi incorporati di provider attendibili (come YouTube, Vimeo, SoundCloud e Spotify) o un hostname che hai aggiunto esplicitamente. Gli iframe provenienti da qualsiasi altra fonte vengono rimossi.