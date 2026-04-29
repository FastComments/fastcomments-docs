Ein **Auslöser** ist ein Ereignis, das einen Agenten aufweckt. Jeder Agent kann einen oder mehrere Auslöser definiert haben.

### Die vollständige Liste

| Auslöser | Wann er ausgelöst wird |
|---|---|
| [Kommentar hinzugefügt](#trigger-comment-add) | Ein neuer Kommentar wird veröffentlicht. |
| [Kommentar bearbeitet](#trigger-comment-edit) | Ein Kommentar wird bearbeitet. Der vorherige Text ist im Kontext des Agenten enthalten. |
| [Kommentar gelöscht](#trigger-comment-delete) | Ein Kommentar wird gelöscht. |
| [Kommentar angepinnt](#trigger-comment-pin) | Ein Kommentar wird angepinnt (von jemandem, einschließlich eines Moderators oder eines anderen Agenten). |
| [Kommentar entpinnt](#trigger-comment-unpin) | Ein Kommentar wird entpinnt. |
| [Kommentar gesperrt](#trigger-comment-lock) | Ein Kommentar wird gesperrt (keine weiteren Antworten erlaubt). |
| [Kommentar entsperrt](#trigger-comment-unlock) | Ein Kommentar wird entsperrt. |
| [Kommentar überschreitet Stimmen-Schwellenwert](#trigger-comment-vote-threshold) | Die Netto-Stimmen eines Kommentars erreichen den konfigurierten Schwellenwert. |
| [Kommentar erreicht Flag-Schwellenwert](#trigger-comment-flag-threshold) | Die Anzahl der Flags eines Kommentars erreicht genau den konfigurierten Schwellenwert. |
| [Benutzer postet ersten Kommentar](#trigger-new-user-first-comment) | Ein Benutzer veröffentlicht seinen ersten Kommentar auf dieser Seite. |
| [Kommentar automatisch als Spam markiert](#trigger-comment-auto-spammed) | Ein Kommentar wird von der Spam-Engine automatisch als Spam markiert. |
| [Moderator überprüft Kommentar](#trigger-moderator-reviewed) | Ein Moderator markiert einen Kommentar als überprüft. |
| [Moderator genehmigt Kommentar](#trigger-moderator-approved) | Ein Moderator genehmigt einen Kommentar. |
| [Moderator markiert Spam](#trigger-moderator-spammed) | Ein Moderator markiert einen Kommentar als Spam. |
| [Moderator verleiht Abzeichen](#trigger-moderator-awarded-badge) | Ein Moderator verleiht einem Benutzer ein Abzeichen. |

### Mehrere Auslöser pro Agent

Ein Agent kann sich auf beliebige Kombinationen von Auslösern abonnieren – die [Moderator-Vorlage](#template-moderator) abonniert beispielsweise sowohl `COMMENT_ADD` als auch `COMMENT_FLAG_THRESHOLD`. Jedes Ereignis löst den Agenten einmal mit dem Kontext dieses Ereignisses aus.

### Was das Auslösen eines Agenten verhindert

Ein abonniertes Auslöser-Ereignis löst den Agenten **nicht** aus, wenn eine der folgenden Bedingungen zutrifft:

- Der [Status](#status-states) des Agenten ist **Deaktiviert**.
- Der [URL- oder Locale-Bereich](#scope-url-locale) des Agenten stimmt nicht mit dem auslösenden Kommentar überein.
- Das [tägliche, monatliche oder Rate-Limit-Budget](#budgets-overview) des Agenten ist erschöpft – der Auslöser wird mit einem Grund als **verworfen** protokolliert. Siehe [Abwurfgründe](#drop-reasons).
- Die Gleichzeitigkeit (Concurrency) für diesen Agenten ist ausgelastet (pro Agent begrenzt).
- Der Tenant des Agenten hat ungültige Abrechnung.
- Die auslösende Aktion wurde selbst von einem Bot oder einem anderen Agenten durchgeführt (Schleifenvermeidung).
- Der Auslöser betraf einen Kommentar, der innerhalb des Deduplicierungsfensters bereits von diesem Agenten verarbeitet wurde.

Wenn ein abonniertes Ereignis erfolgreich ausgelöst wird, zeigt die [Run History](#run-history) des Agenten eine Zeile mit dem Status **Gestartet**, die beim Abschluss des Laufs zu **Erfolgreich** oder **Fehler** wechselt.

### Stimmen- und Flag-Schwellenwerte

Zwei Auslöser – **Comment Crosses Vote Threshold** und **Comment Crosses Flag Threshold** – erfordern einen numerischen Schwellenwert im Bearbeitungsformular. Der Auslöser feuert in dem Moment, in dem die Anzahl den konfigurierten Wert überschreitet (genauer: der Flag-Schwellenwert-Auslöser wird ausgelöst, wenn `flagCount === flagThreshold`, das Wählen von 1 bedeutet also "bei der ersten Meldung auslösen", und das Wählen von 5 bedeutet "auslösen, wenn die fünfte Meldung eintrifft").

### Verzögerte Auslöser

Jeder Auslöser kann verzögert werden, sodass der Agent später ausgeführt wird, zum Beispiel nachdem Stimmen/Flags/Antworten Zeit hatten, sich zu stabilisieren. Siehe [Deferred Triggers](#trigger-deferred-delay).

### Schleifenvermeidung

Um Endlosschleifen zu verhindern, tragen Kommentare, die **von einem Agenten geschrieben wurden**, eine `botId`. New-comment-Auslöser ignorieren Kommentare mit einer `botId`.

Die Konsequenz: Agenten können als Reaktion auf *menschliche* Aktionen in Ihrem Tenant agieren, aber agentenverursachte Aktionen lösen niemals Agenten-Auslöser aus. Dies gilt für alle Auslösertypen.

### REPLAY: der interne Auslöser

Es gibt außerdem einen internen `REPLAY`-Auslösertyp, der von der Funktion [Test Runs (Replays)](#test-runs-replays) verwendet wird. Sie können ihn nicht im Bearbeitungsformular auswählen – er existiert, damit Replay-Läufe in der Run-History deutlich gekennzeichnet und in Live-Run-Ansichten ausgeschlossen werden.