Ci sono molteplici aspetti di sicurezza quando si consente alle persone di aggiungere contenuti a un sito web e poi di renderizzare quel contenuto su molti tipi diversi di dispositivi.

### Prevenzione dell'abuso di formattazione

Le persone possono scrivere contenuti che sono intenzionalmente visivamente distraenti e sminuiscono il valore delle discussioni abusando della formattazione del testo.

FastComments fa diverse cose per prevenire l'abuso riguardo alla formattazione:

- Le grandi interruzioni di riga consecutive ripetute vengono compresse.
- Non renderizziamo i titoli (diventano testo normale).
- Non consentiamo CSS o colori personalizzati.

### Prevenzione degli exploit

Gli exploit possono essere creati nei sistemi che renderizzano HTML. FastComments fa diverse cose per prevenire questo:

- Consentiamo solo un insieme esplicitamente definito di tag HTML.
- Consentiamo solo un insieme esplicitamente definito di attributi dei tag HTML.
- Purifichiamo e sanifichiamo tutti gli input.
  - Questo viene fatto tramite le librerie [DOMPurify](https://www.npmjs.com/package/dompurify) e [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Abbiamo scelto queste librerie perché sono ben testate (con oltre 4 e 1 milione di download a settimana, rispettivamente).

Questo significa che gli utenti non possono fare cose come scrivere tag `<script>` o `<style>`, o tentare di aggiungere script di tipo `onload=alert()` alle immagini o ad altri contenuti.

I tag HTML che consentiamo sono i seguenti:

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
