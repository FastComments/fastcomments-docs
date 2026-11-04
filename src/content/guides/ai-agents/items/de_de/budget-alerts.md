Budget-Alarm-E-Mails werden ausgelöst, wenn die Ausgaben eines Agents einen konfigurierbaren Prozentsatz seines Limits überschreiten. Sie gehen an die Personen, die die Rechnung tragen.

### Wie Alarme funktionieren

Jeder Agent hat ein Feld **Alert thresholds** im Bearbeitungsformular. Standardmäßig sind dies `80%` und `100%`. Sie können einzelne Schwellenwerte an- oder abwählen und weitere Prozentsätze hinzufügen.

Wenn die Ausgaben des Agents in einem bestimmten Zeitraum (täglich oder monatlich) in diesem Zeitraum zum ersten Mal einen Schwellenwert überschreiten, sendet die Plattform eine E-Mail pro Empfänger. Ein erneutes Überschreiten des Schwellenwerts später im selben Zeitraum (z. B. wenn die Ausgaben unter 80 % fielen und wieder darüber stiegen) löst **keine** erneute Zustellung aus.

Dies gilt pro Zeitraum: Ein neuer täglicher Reset startet die Logik zur Erkennung von Schwellenwertüberschreitungen für diesen Tag neu.

### Alarme auf Tenant-Ebene

Der Tenant (das Konto) hat eigene tägliche und monatliche Limits. Alarme auf Tenant-Ebene werden bei festen Schwellenwerten ausgelöst (`80%` und `100%`). Diese sind nicht pro Agent konfigurierbar, da sie für den gesamten Tenant gelten.

### Empfänger

Budget-Alarme werden gesendet an:

- Jeden Benutzer, der auf dem Tenant als **Super admin** markiert ist.
- Jeden Benutzer, der auf dem Tenant als **Billing Admin** markiert ist.

Das umfasst die Vereinigung der beiden Rollen – ein Benutzer mit beiden Rollen erhält eine E-Mail.

### Warum beide Rollen

Super-Admins sind typischerweise die Operatoren, die wissen müssen, wenn ein Agent sein Limit erreicht. Billing-Admins tragen die Rechnung und müssen über Kostenspitzen Bescheid wissen, unabhängig davon, ob sie Agents im Tagesgeschäft verwalten. Um den Agent tatsächlich zu bearbeiten (Limit erhöhen, pausieren), benötigt der Empfänger zusätzlich die Rolle **Customization Admin**, die die Bearbeitungsseite für Agents schützt.

### Opt-out pro Benutzer

Empfänger, die in ihrem Profil Admin-Benachrichtigungen abbestellt haben, werden übersprungen. Dies ist derselbe Opt-out-Schalter, der auch andere Admin-Benachrichtigungen steuert.

Wenn **alle** Empfänger abbestellt haben, wird der Alarm protokolliert (Warnstufe) und keine E-Mail gesendet.

### Inhalt der E-Mail

Die E-Mail enthält:

- Den **Anzeigenamen des Agents** und den internen Namen.
- Den **Zeitraum**, der überschritten wurde (z. B. „agent daily budget", „agent monthly budget", „account daily budget", „account monthly budget").
- Den überschrittenen **Schwellenwert in Prozent**.
- Die **Ausgaben** in der Währung des Tenants.
- Das **Limit** in der Währung des Tenants.
- Einen **One-Click-Signed-Login-Link**, der den Empfänger direkt weiterleitet an:
  - Die Bearbeitungsseite des Agents, bei Alarmen auf Agent-Ebene.
  - Die Listenseite der AI Agents, bei Alarmen auf Tenant-Ebene.

Der Link ist vorauthentifiziert, sodass der Empfänger nur einen Klick davon entfernt ist, das Limit zu erhöhen oder den Agent zu deaktivieren.

### Wie Schwellenwerte ausgelöst werden

Die Plattform verfolgt, welche Schwellenwerte in diesem Zeitraum bereits ausgelöst wurden – getrennt für Agent und Tenant. Das bedeutet:

- Werden 80 % und dann 100 % im selben Zeitraum überschritten, werden beide ausgelöst, in dieser Reihenfolge.
- Ein Sprung direkt von 0 % auf 100 % löst den **höchsten** überschrittenen Schwellenwert (100 %) aus, nicht 80 %, sodass der schwerwiegendste Alarm zugestellt wird.

### Wenn Sie keine Alarme mehr erhalten

Wenn die Ausgaben des Agents in diesem Zeitraum den nächsten Schwellenwert nicht erreichen, erhalten Sie in diesem Zeitraum keine weiteren E-Mails. Der nächste tägliche (oder monatliche) Reset setzt die Verfolgung zurück.

### Alarme deaktivieren

Deaktivieren Sie den Schwellenwert, den Sie nicht möchten. Wenn Sie für einen bestimmten Agent keine Alarme wünschen, deaktivieren Sie alle Prozentsätze. Alarme auf Tenant-Ebene können nicht pro Agent deaktiviert werden (sie gelten tenant-weit).

### Siehe auch

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) – was passiert, wenn das Limit vollständig erreicht ist.
- [Cost Model](#cost-model) – was gemessen wird.