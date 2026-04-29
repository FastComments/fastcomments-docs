Wenn der Agent eine Genehmigung in die Warteschlange stellt, benachrichtigt die Plattform die Prüfer per E-Mail. Zwei Einstellungen im Bearbeitungsformular steuern dies: **wer** benachrichtigt wird und **wie oft**.

### Wer: Benachrichtigungsmodus

Zwei Modi:

- **Alle Administratoren und Moderatoren** (Standard) - jeder Kontoinhaber, Super-Admin und Kommentar-Moderator-Admin beim Mandanten ist ein potenzieller Prüfer.
- **Bestimmte Nutzer** - wählen Sie eine Liste manuell aus einem Dual-List-Picker im Bearbeitungsformular aus.

In beiden Fällen muss ein potenzieller Prüfer ein Konto beim Mandanten und eine gültige E-Mail-Adresse haben, um Benachrichtigungen zu erhalten.

### Wie oft: pro Benutzer

Das **eigene Profil** jedes potenziellen Prüfers legt seine persönliche Benachrichtigungsfrequenz für Agenten-Genehmigungen fest:

- **Sofort** (Standard) - eine E-Mail pro ausstehender Genehmigung, gesendet, sobald die Genehmigung erstellt wird.
- **Stündlich** - eine Zusammenfassungs-E-Mail pro Stunde, die alle in dieser Stunde aufgelaufenen Genehmigungen zusammenfasst.
- **Täglich** - eine Zusammenfassungs-E-Mail alle 24 Stunden.
- **Deaktiviert** - keine E-Mails. Der Benutzer kann Genehmigungen weiterhin über die Inbox-Benutzeroberfläche prüfen; er wird jedoch nicht benachrichtigt.

Der Benutzer ändert diese Einstellung in seinem eigenen Profil, nicht im Agenten-Bearbeitungsformular. Das ist absichtlich so: Ein Mandant könnte zehn Agenten haben, und ein Moderator sollte seine bevorzugte Häufigkeit nicht für jeden Agenten einzeln einstellen müssen.

### Cron-Jobs, die die Zusammenfassungen steuern

- **`hourly-agent-approval-digest`** - durchsucht jede Stunde, bündelt Genehmigungen, die seit der letzten Zusammenfassung jedes Benutzers aufgelaufen sind, und sendet eine E-Mail pro Benutzer.
- **`daily-agent-approval-digest`** - dasselbe, täglich.
- **`agent-approval-reaper`** - entfernt Genehmigungen, die älter als 90 Tage sind, unabhängig vom Status.

Die stündlichen und täglichen Zusammenfassungs-Crons sind empfängerbezogen: Ein Benutzer mit stündlicher Frequenz wird vom stündlichen Cron verarbeitet und vom täglichen übersprungen (und umgekehrt). Benutzer mit Sofort-Frequenz werden über den approval-create Codepfad benachrichtigt, nicht über die Crons.

### Deduplizierungsstatus

Die Plattform verfolgt, welche Benutzer bereits zu jeder Genehmigung per E-Mail benachrichtigt wurden. Sobald ein Benutzer benachrichtigt wurde (sofort oder in einer Zusammenfassung), wird er für dieselbe Genehmigung nicht erneut per E-Mail kontaktiert – selbst wenn er seine Frequenz während des Zyklus von Sofort auf Täglich ändert.

### Genehmigung per E-Mail

Jede Benachrichtigungs-E-Mail enthält einen signierten Ein-Klick-Login-Link, der den Prüfer direkt zur Detailseite der Genehmigung führt, bereits authentifiziert. Von dort aus können sie genehmigen, ablehnen oder den [Prompts verfeinern](#refining-prompts)-Flow öffnen.

### Was passiert, wenn keine Administratoren vorhanden sind

Wenn `notifyMode` `All admins and moderators` ist, der Mandant jedoch keine Super-Admins, Kommentar-Moderator-Admins oder Kontoinhaber mit gültigen E‑Mail-Adressen hat, protokolliert die Plattform eine Warnung und die Genehmigung wird trotzdem in die Warteschlange gestellt – nur wird niemand darüber benachrichtigt. Sie bleibt im Posteingang, bis jemand zufällig nachsieht.

Wenn `notifyMode` `Specific users` ist, Sie aber keine Benutzer ausgewählt haben, dasselbe Ergebnis.

### Was passiert, wenn Abrechnungsbenachrichtigungen deaktiviert sind

[Budgetwarnungen](#budget-alerts) - die budgetbezogenen E-Mails - gehen an die Abrechnungs-Admins **unabhängig von den benutzerbezogenen Benachrichtigungseinstellungen**. Das ist beabsichtigt: Budgetüberschreitungen betreffen die Kosten, und der Abrechnungsverantwortliche muss informiert werden.

Genehmigungsbenachrichtigungen respektieren nur die benutzerbezogene Einstellung zur Agenten-Genehmigungsfrequenz. Sie berücksichtigen nicht das allgemeine Abmeldeverhalten für Administratorbenachrichtigungen – ein Benutzer, der sich von Administratorbenachrichtigungen abgemeldet hat, erhält weiterhin Genehmigungs-E-Mails, wenn er in der Prüferliste steht, es sei denn, seine Agenten-Genehmigungsfrequenz ist auf **Deaktiviert** gesetzt.

### Siehe auch

- [Genehmigungsablauf](#approval-workflow) für den gesamten Lebenszyklus einer Genehmigung.
- [Prompts verfeinern](#refining-prompts) für den Workflow „Ich genehmige immer wieder die gleiche Art von Fehler“.