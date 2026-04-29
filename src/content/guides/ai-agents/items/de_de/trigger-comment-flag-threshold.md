Wird ausgelöst, wenn die Anzahl der Flags eines Kommentars **genau** den konfigurierten Schwellenwert erreicht.

### Erforderliche Konfiguration

- **Flag threshold** - integer >= 1. Der Trigger feuert in dem Moment, in dem `flagCount === flagThreshold`. Er wird bei weiteren Flags oberhalb des Schwellenwerts nicht erneut ausgelöst.

Wenn der Schwellenwert 3 ist und drei Nutzer den Kommentar flaggen, wird der Agent beim dritten Flag einmal ausgelöst. Ein viertes, fünftes oder sechstes Flag löst ihn **nicht** erneut aus.

### Kontext, den der Agent erhält

- Der markierte Kommentar.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext, wie konfiguriert.
- Die Flag-Anzahl steht im Kommentarblock als `Flag Count: N`.

### Bemerkenswert

- Der Trigger wird nur ausgelöst, wenn der Kommentar die Schwelle von unten über den Flag-Handling-Pfad der Plattform überschreitet (wo `didIncrement === true`). Direkte DB-Schreibvorgänge, die `flagCount` auf den Schwellenwert setzen, lösen ihn nicht aus; Flags oberhalb des Schwellenwerts lösen ihn ebenfalls nicht erneut aus.
- Es enthält nicht, wer den Kommentar geflaggt hat – Flags sind für den Agenten anonym. Wenn Sie die flaggenden Nutzer betrachten möchten, holen Sie diese aus Ihren eigenen Daten.
- Eine Trigger-Verzögerung (siehe [Verzögerte Auslöser](#trigger-deferred-delay)) wird für diesen Trigger *dringend* empfohlen – Flags treffen in hitzigen Threads häufig in Schüben ein, und eine kleine Verzögerung lässt die Lage sich klären, bevor der Agent handelt.

### Häufige Verwendungszwecke

- **Moderationsprüfung** - ein markierter Kommentar ist das klassische Signal „Menschen denken, das könnte problematisch sein“. Die [Moderator-Vorlage](#template-moderator) abonniert diesen Trigger standardmäßig mit einem Flag-Schwellenwert von 3.
- **Erweiterung der Pre-Moderation-Warteschlange** - Der Agent führt einen ersten Durchgang aus und markiert entweder den Kommentar zur Moderation (mit `mark_comment_reviewed`) oder eskaliert weiter.
- **Anti-Brigading** - Kombinieren Sie diesen Trigger mit dem [Benutzerverlauf-Kontext](#context-options) und lassen Sie den Agenten vorherige Sperren/Duplikat-Inhalts-Signale sehen, bevor er handelt.

### Kombinationsempfehlungen

Abonnieren Sie **sowohl** `COMMENT_ADD` als auch `COMMENT_FLAG_THRESHOLD`, wenn Sie einen Moderationsagenten möchten, der offensichtliche Fälle auf den ersten Blick erfasst und Grenzfälle neu bewertet, sobald Flags sich anhäufen. Die beiden Events werden unabhängig voneinander ausgelöst – der Agent läuft zweimal, wenn beide abonniert sind und beide feuern, aber der zweite Durchlauf sieht den nun markierten Zustand.