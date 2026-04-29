Wird ausgelöst, wenn die Netto-Stimmenanzahl eines Kommentars den konfigurierten Schwellenwert erreicht. Netto-Stimmen sind `votesUp - votesDown`.

### Erforderliche Konfiguration

- **Vote threshold** - Ganzzahl >= 1. Der Trigger wird bei der Abstimmung ausgelöst, die die Netto-Stimmen genau auf diesen Wert bringt.

Wenn der Schwellenwert 10 ist und ein Kommentar von 9 auf 10 Netto-Stimmen steigt, wird der Trigger einmal ausgelöst. Führt eine weitere Abstimmung den Wert von 10 auf 11, wird der Trigger **nicht** erneut ausgelöst — er feuert nicht bei jeder zusätzlichen Stimme über dem Schwellenwert.

### Kontext, den der Agent erhält

- Der Kommentar mit den aktuellen Stimmenzahlen.
- Die **Richtung der Stimme** (`up` oder `down`), die das Überschreiten des Schwellenwerts ausgelöst hat.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext wie konfiguriert.

### Bemerkungen

- Ein Kommentar, der auf 10 steigt, wieder auf 9 zurückfällt und erneut auf 10 steigt, löst den Trigger zweimal aus. Es gibt keinen pro-Kommentar-Status "bereits ausgelöst" — wenn Sie diese Semantik benötigen, lassen Sie den Agenten beim ersten Lauf eine [memory note](#tools-overview) speichern und prüfen Sie diese bei nachfolgenden Läufen.
- Der Schwellenwert ist immer eine **Netto**-Stimmenanzahl, nicht rohe Upvotes. Ein Kommentar mit 12 up und 2 down hat netto 10 und löst den Trigger aus; einer mit 10 up und 0 down tut das ebenfalls.
- Auch Überschreitungen, die nur durch Downvotes entstehen, sind möglich — ein Kommentar, der durch einen Down-Vote von 11 auf 10 fällt, löst ebenfalls aus. Der `voteDirection`-Parameter im Kontext teilt dem Agenten mit, aus welcher Richtung die Schwellenwertüberschreitung kam.

### Häufige Anwendungsfälle

- **Pinning** - die [Top Comment Pinner template](#template-top-comment-pinner) ist um diesen Trigger herum aufgebaut.
- **Promotion / featured comment workflows** - sende ein Ereignis über [Webhooks](#webhooks-overview), damit ein externes System den Kommentar an anderer Stelle Ihrer Website bewerben kann.
- **Engagement tracking** - speichere eine memory note über den Benutzer, der den Kommentar geschrieben hat, damit andere Agenten wissen, dass er populären Inhalt erstellt hat.

### Feinabstimmung

Der richtige Schwellenwert ist community-spezifisch. Beobachten Sie die [Run History](#run-history) einige Tage lang mit einem niedrigen Schwellenwert (5), um zu sehen, wie oft er ausgelöst wird. Erhöhen Sie den Schwellenwert, bis die Auslösehäufigkeit der gewünschten Frequenz entspricht.