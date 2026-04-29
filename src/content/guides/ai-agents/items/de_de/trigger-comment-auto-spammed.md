Wird ausgelöst, wenn ein Kommentar vom integrierten Spam-Mechanismus von FastComments automatisch als Spam markiert wird - **nicht** von einem Moderator und nicht von einem anderen Agenten.

### Kontext, den der Agent erhält

- Der automatisch als Spam markierte Kommentar.
- Optionaler Thread-/Benutzerverlauf/Seitenkontext wie konfiguriert.

### Wer löst dies aus

Die Spam-Pipeline der Plattform. Siehe [Spam-Erkennung](/guide-moderation.html#spam-detection) im Moderationsleitfaden für weitere Details.

### Häufige Verwendungszwecke

- **Zweitprüfung** - die Spam-Engine hat eine hohe Trefferquote (Recall), jedoch keine perfekte Präzision; ein Agent, der auf den spezifischen Stil Ihrer Community trainiert ist, kann falsch positive Ergebnisse erkennen. Der Agent kann aufrufen, um einen fälschlich klassifizierten Kommentar zu ent-flaggen.
- **Automatisiertes Entbannen** - wenn Ihr Mandant neue Konten aggressiv wegen Spam sperrt, kann ein Agent bei diesem Trigger offensichtliche Fehlklassifikationen überprüfen und aufheben, bevor ein Mensch sie überhaupt sieht.

### Bemerkenswert

- Der Trigger wird **nicht** ausgelöst bei von Moderatoren als Spam markierten Kommentaren (verwenden Sie [Trigger: Moderator markierter Spam](#trigger-moderator-spammed)) noch bei Spam, der von einem anderen Agenten markiert wurde.
- Ein Kommentar, der automatisch als Spam markiert wurde und später von einem Moderator als Nicht-Spam markiert wird, löst den Trigger nicht erneut aus.
- Das Abonnieren dieses Triggers ist am nützlichsten in Mandanten, in denen der Auto-Spam-Modus der Engine in den Moderationseinstellungen aktiviert ist. Andernfalls wird der Trigger nicht ausgelöst.