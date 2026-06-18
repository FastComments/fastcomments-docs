Beim Zulassen von Inhalten durch Nutzer auf einer Website und deren Darstellung auf vielen verschiedenen Gerätetypen gibt es mehrere Sicherheitsaspekte.

### Verhinderung von Formatierungsmissbrauch

Nutzer können Inhalte verfassen, die durch missbräuchliche Textformatierung absichtlich optisch ablenken und den Wert von Diskussionen mindern.

FastComments unternimmt mehrere Maßnahmen, um Missbrauch bei der Formatierung zu verhindern:

- Große, wiederholt aufeinanderfolgende Zeilenumbrüche werden zusammengeführt.
- Wir rendern keine Überschriften (sie werden zu normalem Text).
- Wir erlauben kein CSS oder benutzerdefinierte Farben.

### Schutz vor Exploits

In Systemen, die HTML rendern, können Exploits entstehen. FastComments ergreift mehrere Maßnahmen, um dies zu verhindern:

- Wir erlauben nur eine explizit definierte Menge von HTML-Tags.
- Wir erlauben nur eine explizit definierte Menge von HTML-Tag-Attributen.
- Wir bereinigen und säubern alle Eingaben.
  - Dies erfolgt mithilfe der Bibliotheken [DOMPurify](https://www.npmjs.com/package/dompurify) und [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Wir haben diese Bibliotheken gewählt, weil sie gut getestet sind (mit über 4 bzw. 1 Million Downloads pro Woche).

Das bedeutet, dass Nutzer nicht etwas wie `<script>`- oder `<style>`-Tags schreiben können, oder versuchen, `onload=alert()`-artige Skripte zu Bildern oder anderem Inhalt hinzuzufügen.

Die von uns erlaubten HTML-Tags sind wie folgt:

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

Der `<iframe>`-Tag ist standardmäßig nicht erlaubt. Wenn Sie Medieneinbettungen zulassen, sind iframes ebenfalls erlaubt, aber nur, wenn deren Quelle einer der eingebauten vertrauenswürdigen Anbieter (wie YouTube, Vimeo, SoundCloud und Spotify) oder einem von Ihnen explizit hinzugefügten Hostnamen entspricht. Iframes von anderen Quellen werden entfernt.