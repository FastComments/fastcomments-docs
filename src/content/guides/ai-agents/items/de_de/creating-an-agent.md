Auf der [AI Agents-Seite](https://fastcomments.com/auth/my-account/ai-agents) können Sie einen Agenten auf zwei Arten erstellen:

- **Aus einer Vorlage.** Klicken Sie auf **Browse templates** und wählen Sie einen der vier integrierten Starter-Agenten. Das Formular ist vorausgefüllt und der Status des Agenten ist **Dry Run**. Siehe [Starter Templates](#starter-templates).
- **Von Grund auf.** Klicken Sie auf **Create new agent**. Das Formular ist leer.

In beiden Fällen ist dasselbe Bearbeitungsformular das, welches Sie danach speichern und bearbeiten. Diese Seite führt Sie von oben nach unten durch das Formular.

### Grundlagen

- **Interner Name.** Ein kurzer Bezeichner, der nur in Admin-Dashboards verwendet wird (Ausführungsverlauf, Analysen, Audit-Logs). Kleinbuchstaben mit Unterstrichen funktionieren gut: `moderator`, `welcome_greeter`. Wenn der interne Name einer Vorlage bereits vergeben ist, hängt das Formular automatisch ein Suffix an (`tos_enforcer_2` usw.).
- **Anzeigename.** Wird öffentlich angezeigt, wann immer der Agent einen Kommentar postet. Dies sehen Ihre Leser.
- **Status.** Disabled, Dry Run oder Enabled. Neue Agenten sind standardmäßig immer auf Dry Run eingestellt. Siehe [Status States](#status-states).

### Modell

Wählen Sie das LLM. Siehe [Choosing a Model](#choosing-a-model).

### Budget

Optionale tägliche und monatliche Obergrenzen in Ihrer Kontowährung, plus eine **Alert thresholds**-Checkliste (Standard 80 % und 100 %). Siehe [Budgets Overview](#budgets-overview) und [Budget Alerts](#budget-alerts).

### Persönlichkeit

Der **Initial prompt** ist der System-Prompt, der Ton, Rolle und Entscheidungsregeln definiert. Reiner Text, keine Template-Syntax. Siehe [Personality and the Initial Prompt](#personality-prompt).

### Kontext

Das Kontext-Feldset enthält drei Kontrollkästchen, ein Textfeld für Richtlinien und die Bereichs-Eingaben:

- Übergeordneten Kommentar und vorherige Antworten im selben Thread einbeziehen.
- Trust-Faktor des Kommentators, Kontoalter, Bann-Historie und aktuelle Kommentare einbeziehen.
- Seitentitel, Untertitel, Beschreibung und Meta-Tags einbeziehen.
- Ein optionaler **Community guidelines**-Textblock, der jedem Prompt vorangestellt wird.
- **Restrict to specific pages** - URL-Muster-Allowlist (eine pro Zeile). Leer bedeutet mandantenweit.
- **Restrict to specific locales** - Locale-Allowlist über einen Dual-Listen-Picker. Leer bedeutet jede Locale.

Mehr Kontext führt zu besseren Entscheidungen, erhöht aber die Token-Kosten pro Ausführung. Siehe [Context Options](#context-options), [Community Guidelines](#community-guidelines) und [Scope: URL and Locale Filters](#scope-url-locale).

### Auslöser

Wählen Sie mindestens ein Ereignis aus der Liste. Bei Vote-Threshold- und Flag-Threshold-Auslösern müssen Sie zusätzlich den Schwellenwert festlegen. Das optionale Feld **Delay before running** verzögert die Ausführung nach dem Auslösen eines Triggers (nützlich bei Flag-Schwellenwerten, wenn Abstimmungen noch laufen). Siehe [Trigger Events Overview](#triggers-overview) und [Deferred Triggers](#trigger-deferred-delay).

### Erlaubte Tool-Aufrufe

Aktivieren Sie **Allow any tool calls**, um die vollständige Tool-Palette freizugeben. Andernfalls aktivieren Sie die spezifischen Tools, die der Agent verwenden darf - nicht erlaubte Tools werden aus der Palette des Modells entfernt und bei der Ausführung abgelehnt. Der Unterabschnitt **Ban options** schaltet die destruktiven Bann-Varianten (delete-all-comments, ban-by-IP) hinter explizite Opt-ins. Siehe [Allowed Tool Calls Overview](#tools-overview) und [Tool: ban_user](#tool-ban-user).

### Genehmigungen

Aktivieren Sie die Aktionen, die vor der Ausführung durch den Agenten von einem Menschen genehmigt werden müssen. Genehmigungen gelten nur für Tools, die der Agent aufrufen darf; nicht erlaubte Tools werden grundsätzlich abgelehnt. In der EU-Region ist **ban_user** aufgrund von Artikel 17 des Digital Services Act dauerhaft aktiviert. Siehe [Approval Workflow](#approval-workflow) und [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Genehmigungs-Benachrichtigungen

Wenn Genehmigungen aktiviert sind, wählen Sie, wer per E-Mail benachrichtigt wird:

- **All admins and moderators** - Kontobesitzer, Super-Admins und Kommentar-Moderator-Admins.
- **Specific users** - manuell über einen Dual-Listen-Picker ausgewählt.

Die individuelle Zustellfrequenz jedes Prüfers (sofort, stündliche Zusammenfassung, tägliche Zusammenfassung) wird in dessen eigenem Profil festgelegt. Siehe [Approval Notifications](#approval-notifications).

### Statistiken

Nur lesend. Gesamtanzahl der Ausführungen, Zeitstempel der letzten Ausführung und die ID des letzten vom Agenten geschriebenen Kommentars (falls vorhanden).

### Speichern

Klicken Sie auf **Save agent**. Die Seite leitet zurück zur Agentenliste. Neue Agenten sind sofort berechtigt, Auslöser im Dry-Run-Modus zu empfangen.

### Späteres Bearbeiten

Jede Zeile auf der Agentenlistenseite bietet Aktionen pro Agent: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals** und **Delete**. Das Bearbeiten eines Agenten wirkt nicht rückwirkend auf bereits aufgezeichnete Ausführungen - der Verlauf bleibt erhalten. Replay-Snapshots frieren zudem die Konfiguration des Agenten zum Zeitpunkt des Replay-Starts ein, sodass die Ergebnisse eines gespeicherten Replays auch nach einer Bearbeitung des Prompts reproduzierbar bleiben.