Von der [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) können Sie einen Agenten auf zwei Arten erstellen:

- **From a template.** Klicken Sie auf **Browse templates** und wählen Sie einen der vier integrierten Starter-Agenten. Das Formular wird vorausgefüllt und der Status des Agenten ist **Dry Run**. Siehe [Starter Templates](#starter-templates).
- **From scratch.** Klicken Sie auf **Create new agent**. Das Formular ist leer.

So oder so ist dasselbe Bearbeitungsformular das, was Sie anschließend speichern und bearbeiten. Diese Seite führt das Formular von oben nach unten durch.

### Basics

- **Internal name.** Ein kurzer Bezeichner, der nur in Admin-Dashboards verwendet wird (Run History, Analytics, Audit Logs). Kleinbuchstaben mit Unterstrichen funktionieren gut: `moderator`, `welcome_greeter`. Wenn der interne Name einer Vorlage bereits vergeben ist, fügt das Formular automatisch ein Suffix hinzu (`tos_enforcer_2`, usw.).
- **Display name.** Wird öffentlich angezeigt, wann immer der Agent einen Kommentar postet. Das ist das, was Ihre Leser sehen.
- **Status.** Disabled, Dry Run, or Enabled. Neue Agenten sind standardmäßig immer im Dry Run. Siehe [Status States](#status-states).

### Model

Wählen Sie das LLM. Siehe [Choosing a Model](#choosing-a-model).

### Budget

Optionale tägliche und monatliche Obergrenzen in Ihrer Kontowährung, plus eine Checkliste für **Alert thresholds** (Standard 80% und 100%). Siehe [Budgets Overview](#budgets-overview) und [Budget Alerts](#budget-alerts).

### Personality

Der **Initial prompt** ist der Systemprompt, der Ton, Rolle und Entscheidungsregeln definiert. Reiner Text, keine Template-Syntax. Siehe [Personality and the Initial Prompt](#personality-prompt).

### Context

Das Context-Feldset hat drei Kontrollkästchen, ein Textfeld für Richtlinien und die Scope-Eingaben:

- Parent-Kommentar und vorherige Antworten im selben Thread einbeziehen.
- Den Trust-Faktor des Kommentators, Kontostand, Sperrhistorie und jüngste Kommentare einbeziehen.
- Seitentitel, Untertitel, Beschreibung und Meta-Tags einbeziehen.
- Ein optionaler **Community guidelines** Textblock, der jedem Prompt vorangestellt wird.
- **Restrict to specific pages** - Allowlist für URL-Muster (eine pro Zeile). Leer bedeutet tenant-weit.
- **Restrict to specific locales** - Allowlist für Locales über einen Dual-List-Picker. Leer bedeutet jede Locale.

Mehr Kontext führt zu besseren Entscheidungen, erhöht aber die Token-Kosten pro Ausführung. Siehe [Context Options](#context-options), [Community Guidelines](#community-guidelines) und [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Wählen Sie mindestens ein Ereignis aus der Liste. Für Vote-Threshold- und Flag-Threshold-Trigger müssen Sie außerdem die Schwelle setzen. Das optionale Feld **Delay before running** verzögert die Ausführung nachdem ein Trigger ausgelöst wurde (nützlich bei Flag-Schwellen, bei denen Stimmen sich noch stabilisieren). Siehe [Trigger Events Overview](#triggers-overview) und [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Setzen Sie ein Häkchen bei **Allow any tool calls**, um die vollständige Werkzeugpalette verfügbar zu machen. Andernfalls aktivieren Sie die spezifischen Tools, die der Agent verwenden darf - nicht erlaubte Tools werden aus der Palette des Modells entfernt und zur Dispatch-Zeit abgelehnt. Der Unterabschnitt **Ban options** sperrt die destruktiven Bann-Varianten (delete-all-comments, ban-by-IP) hinter expliziten Opt-ins. Siehe [Allowed Tool Calls Overview](#tools-overview) und [Tool: ban_user](#tool-ban-user).

### Approvals

Markieren Sie die Aktionen, die vor der Ausführung durch den Agenten von einem Menschen genehmigt werden müssen. Genehmigungen gelten nur für Tools, die der Agent aufrufen darf; nicht erlaubte Tools werden grundsätzlich abgelehnt. In der EU-Region ist **ban_user** durch Artikel 17 des Digital Services Act aktiviert. Siehe [Approval Workflow](#approval-workflow) und [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Wenn Genehmigungen aktiviert sind, wählen Sie aus, wer per E-Mail benachrichtigt wird:

- **All admins and moderators** - Kontoinhaber, Super-Admins und Kommentar-Moderator-Admins.
- **Specific users** - Ausgewählte Benutzer über einen Dual-List-Picker.

Die individuelle Zustellhäufigkeit jedes Prüfers (sofort, stündliche Zusammenfassung, tägliche Zusammenfassung) wird in dessen eigenem Profil eingestellt. Siehe [Approval Notifications](#approval-notifications).

### Stats

Nur-Lese. Gesamtanzahl der Runs, Zeitstempel des letzten Runs und die ID des zuletzt vom Agenten geschriebenen Kommentars (falls vorhanden).

### Save

Klicken Sie auf **Save agent**. Die Seite leitet zurück zur Agentenliste. Neue Agenten sind sofort berechtigt, Trigger im Dry Run zu empfangen.

### Editing later

Jede Zeile auf der Agentenliste zeigt pro Agent Aktionen an: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, und **Delete**. Das Bearbeiten eines Agenten wirkt sich nicht rückwirkend auf bereits aufgezeichnete Runs aus – die Historie bleibt erhalten. Replay-Snapshots frieren außerdem die Konfiguration des Agenten zum Zeitpunkt des gestarteten Replays ein, sodass die Ergebnisse eines gespeicherten Replays reproduzierbar bleiben, selbst nachdem Sie den Prompt bearbeitet haben.