---
Standardmäßig übersetzt FastComments die eigene Benutzeroberfläche des Widgets (Schaltflächen, Beschriftungen, Fehlermeldungen) in die Sprache des Betrachters, aber Kommentare werden genau so angezeigt, wie sie geschrieben wurden.

Die Kommentarübersetzung fügt die Übersetzung der Kommentare selbst hinzu, sodass eine Community, die in mehr als einer Sprache schreibt, jeden Kommentar in ihrer eigenen Sprache lesen kann.

Es gibt zwei Modi:

- **Automatic**: Kommentare, die in einer anderen Sprache als der des Widgets geschrieben wurden, werden übersetzt angezeigt. Eine Zeile „Übersetzt aus Spanisch“ erscheint unter dem Kommentar mit einem **Original anzeigen**‑Link, der zwischen der Übersetzung und dem unveränderten Text umschaltet.
- **Manual**: Jeder Kommentar erhält einen **Übersetzen**‑Link. Durch Klicken wird dieser einzelne Kommentar in die Sprache des Betrachters übersetzt, mit demselben **Original anzeigen**‑Umschalter danach. Kommentare, die bereits in der Sprache des Betrachters verfasst sind, geben dies stattdessen an.

Die Zielsprache ist die Sprache des Widgets: die [Locale / Language](#locale) Einstellung in derselben Anpassungsregel, falls sie gesetzt ist, andernfalls die Browsersprache des Besuchers. Vom Besucher ist nichts erforderlich, kein Konto, keine Erweiterung oder zusätzliche Software, und es funktioniert sowohl im eingebetteten Widget als auch auf eigenständigen Kommentarseiten gleich.

Dies wird pro <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> aktiviert, sodass es auf Ihre gesamte Website, eine Domain oder eine einzelne Seite angewendet werden kann. Standardmäßig ist es deaktiviert.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Dropdown für Kommentarübersetzung auf der Widget-Anpassungsseite mit den Optionen Aus, Automatisch und Manuell'; title='Kommentarübersetzung' app-screenshot-end]

Übersetzung ist eine serverseitige Einstellung. Das Übergeben von `commentTranslation` in der JavaScript-Konfiguration des Widgets aktiviert sie nicht; die Regel tut es.

Jeder Kommentar wird höchstens einmal pro Zielsprache übersetzt und das Ergebnis zusammen mit dem Kommentar gespeichert, sodass ein Kommentar, der von tausend spanischsprachigen Lesern gelesen wird, nur einmal übersetzt wird. Wenn der Autor oder ein Moderator den Kommentar bearbeitet, wird die gespeicherte Übersetzung verworfen und der Kommentar beim nächsten Aufruf erneut übersetzt. Übersetzter Text durchläuft dieselbe Bereinigung und dieselben Linkregeln wie der Originalkommentar. Kommentare, die zu lang sind oder deren Sprache nicht erkannt werden kann, werden unverändert angezeigt.

Die Nutzung wird als LLM‑Token zum gleichen Tarif wie [AI Agents](/guide-ai-agents.html#plans-and-eligibility) abgerechnet und erscheint an derselben Stelle in Ihren Abrechnungsdetails.
---