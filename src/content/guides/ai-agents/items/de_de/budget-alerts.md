Budget-Warn-E-Mails werden ausgelöst, wenn die Ausgaben eines Agenten einen konfigurierbaren Prozentsatz seines Limits überschreiten. Sie gehen an die Personen, die die Rechnung tragen.

### How alerts work

Jeder Agent hat im Bearbeitungsformular ein Feld **Alert thresholds**. Standardmäßig ist es `80%` und `100%`. Sie können einzelne Schwellenwerte an- oder abwählen und weitere Prozentsätze hinzufügen.

Wenn die Ausgaben des Agenten in einem bestimmten Zeitraum (täglich oder monatlich) zum ersten Mal in diesem Zeitraum einen Schwellenwert überschreiten, sendet die Plattform eine E-Mail pro Empfänger. Das erneute Überschreiten desselben Schwellenwerts später im selben Zeitraum (z. B. die Ausgaben fielen unter 80% und stiegen wieder darüber) sendet **keine** weitere E-Mail.

Dies gilt pro Zeitraum: Ein neuer täglicher Reset startet die Logik für das Überschreiten von Schwellenwerten an diesem Tag neu.

### Tenant-scope alerts

Der Tenant (Account) hat eigene tägliche und monatliche Limits. Tenant-scope Alerts werden bei festen Schwellenwerten (`80%` und `100%`) ausgelöst. Diese sind nicht pro Agent konfigurierbar, da sie für den gesamten Tenant gelten.

### Recipients

Budget-Warnungen werden gesendet an:

- Jeden Nutzer, der auf dem Tenant als **Super admin** markiert ist.
- Jeden Nutzer, der auf dem Tenant als **Billing Admin** markiert ist.

Das beinhaltet die Vereinigung der beiden Rollen – ein Nutzer mit beiden Rollen erhält nur eine E-Mail.

### Why both roles

Super admins sind typischerweise die Betreiber, die wissen müssen, dass ein Agent sein Limit erreicht. Billing admins sind für die Rechnung verantwortlich und müssen über Kostenanstiege informiert werden, unabhängig davon, ob sie Agenten im Tagesgeschäft verwalten. Um den Agenten tatsächlich zu bearbeiten (das Limit zu erhöhen, ihn zu pausieren), benötigt der Empfänger außerdem die Rolle **Customization Admin** – diese schränkt die Bearbeitungsseite des Agenten ab.

### Per-user opt-out

Empfänger, die in ihrem Profil Admin-Benachrichtigungen abgewählt haben, werden übersprungen. Dies ist derselbe Opt-out-Schalter, der andere Admin-Benachrichtigungen steuert.

Wenn **alle** Empfänger abgewählt sind, wird die Warnung protokolliert (Warnstufe) und es wird keine E-Mail gesendet.

### Email content

Die E-Mail enthält:

- Den **Agent display name** und den internen Namen.
- Den **scope**, der überschritten wurde (z. B. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Den **überschrittenen Schwellenwert in Prozent**.
- **Verbrauch** in der Währung des Tenants.
- **Limit** in der Währung des Tenants.
- Einen **einmalig signierten Login-Link mit einem Klick**, der den Empfänger direkt führt zu:
  - Der Agent-Bearbeitungsseite, bei agent-scope Alerts.
  - Der AI Agents-Listenansicht, bei tenant-scope Alerts.

Der Link ist vorautorisierend, sodass der Empfänger mit einem Klick das Limit erhöhen oder den Agenten deaktivieren kann.

### How thresholds fire

Die Plattform verfolgt, welche Schwellenwerte in diesem Zeitraum bereits ausgelöst wurden, getrennt für den Agenten und den Tenant. Daher:

- Das Überschreiten von 80% und dann 100% im selben Zeitraum löst beide aus, in dieser Reihenfolge.
- Ein direktes Springen von 0% auf 100% in einem großen Sprung löst den **höchsten** überschrittenen Schwellenwert (100%) aus, nicht 80%, sodass die schwerwiegendste Warnung ausgeliefert wird.

### When you stop getting alerts

Wenn die Ausgaben des Agenten diesen Zeitraum nie den nächsten Schwellenwert erreichen, erhalten Sie in diesem Zeitraum keine weiteren E-Mails. Der nächste tägliche Reset (oder monatliche Reset) löscht die Verfolgung.

### Disabling alerts

Wählen Sie den Schwellenwert ab, den Sie nicht möchten. Wenn Sie für einen bestimmten Agenten keine Warnungen wünschen, wählen Sie alle Prozentsätze ab. Tenant-scope Alerts können nicht pro Agent deaktiviert werden (sie gelten tenantweit).

### See also

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - was passiert, wenn das Limit vollständig erreicht ist.
- [Cost Model](#cost-model) - was gemessen wird.