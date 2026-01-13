Es gibt mehrere Aspekte der Sicherheit, wenn man Menschen erlaubt, Inhalte zu einer Website hinzuzufügen und diese Inhalte dann auf vielen verschiedenen Gerätetypen anzuzeigen.

### Verhinderung von Formatierungsmissbrauch

Menschen können Inhalte schreiben, die absichtlich visuell ablenkend sind und den Wert von Diskussionen durch Missbrauch von Textformatierung mindern.

FastComments unternimmt eine Reihe von Maßnahmen, um Missbrauch in Bezug auf Formatierung zu verhindern:

- Große wiederholte aufeinanderfolgende Zeilenumbrüche werden zusammengefasst.
- Wir rendern keine Überschriften (sie werden zu normalem Text).
- Wir erlauben kein CSS oder benutzerdefinierte Farben.

### Verhinderung von Exploits

Exploits können in Systemen erstellt werden, die HTML rendern. FastComments unternimmt mehrere Maßnahmen, um dies zu verhindern:

- Wir erlauben nur einen explizit definierten Satz von HTML-Tags.
- Wir erlauben nur einen explizit definierten Satz von HTML-Tag-Attributen.
- Wir bereinigen und sanitisieren alle Eingaben.
  - Dies geschieht über die Bibliotheken [DOMPurify](https://www.npmjs.com/package/dompurify) und [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Wir haben diese Bibliotheken gewählt, da sie gut getestet sind (mit über 4 bzw. 1 Million Downloads pro Woche).

Dies bedeutet, dass Benutzer keine Dinge wie `<script>`- oder `<style>`-Tags schreiben oder versuchen können, `onload=alert()`-Skripte zu Bildern oder anderen Inhalten hinzuzufügen.

Die HTML-Tags, die wir erlauben, sind wie folgt:

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
