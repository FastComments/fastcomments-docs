Wird ausgelöst, wenn ein Kommentar gelöscht wird.

### Kontext, den der Agent erhält

- Der soeben gelöschte Kommentar - Text, Autor, Seite.
- Optionaler Thread / Benutzerverlauf / Seitenkontext, wie konfiguriert.

### Bemerkenswert

- Wird sowohl bei **weichen Löschungen** (bei denen der Kommentar ausgeblendet, aber zur Prüfung erhalten bleibt) als auch bei **harten Löschungen** (bei denen der Kommentar vollständig entfernt wird) ausgelöst. Der Trigger-Handler löst den Kommentar aus der Pipeline für kaskadierende Löschvorgänge auf; was der Agent sieht, ist der zuletzt bekannte Zustand.
- Sobald ein Kommentar vollständig gelöscht ist, schlagen Tools, die darauf zielen (`pin_comment`, `mark_comment_spam`, etc.) für diese Kommentar-ID fehl.

### Häufige Verwendungszwecke

- **Audit-Weiterleitung über [Webhooks](#webhooks-overview)** - sende ein `trigger.succeeded`-Ereignis, damit ein externes System aufzeichnet, was gelöscht wurde.
- **Speichervorgänge** - lasse den Agenten eine [Memory-Notiz](#tools-overview) über ein Löschmuster aufzeichnen (der gelöschte Kommentar war z. B. der dritte des Nutzers innerhalb von 24 Stunden, usw.).
- **Auswirkungen auf andere Threads** - erkenne, wenn eine Löschung die Struktur eines Threads ändert, den der Agent zuvor zusammengefasst hat, und überlege, ob eine erneute Zusammenfassung notwendig ist.

### Hinweis zur Betriebsbelastung

Wenn Sie eine Website mit hoher Löschrate haben (intensive manuelle Moderation), kann dieser Trigger häufig ausgelöst werden.

---