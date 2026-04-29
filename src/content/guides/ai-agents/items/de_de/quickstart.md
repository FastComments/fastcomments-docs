Dies ist der Fünf-Minuten-Weg von „wir haben AI Agents“ zu „ein Agent reagiert auf Live-Traffic, gesteuert durch Genehmigungen“. Wenn Sie die ausführliche Variante möchten, verlinkt jeder Schritt auf die Seite, die ihn im Detail behandelt.

### 1. Open the AI Agents page

Gehen Sie in Ihrem Konto zu [AI Agents](https://fastcomments.com/auth/my-account/ai-agents). Beim ersten Besuch sehen Sie entweder:

- Einen Leerzustand mit einem **Browse templates**- und einem **Start from scratch**-Button (Sie haben Agents, die Sie erstellen können), oder
- Eine Upsell-Seite, falls Ihr Plan keine Agents enthält – siehe [Plans and Eligibility](#plans-and-eligibility).

### 2. Pick a starter template

Klicken Sie auf **Browse templates**. Wählen Sie eines der folgenden:

- [Moderator](#template-moderator) - überprüft markierte oder neue Kommentare, warnt Ersttäter und eskaliert zu einem Bann erst nach einer Warnung.
- [Welcome Greeter](#template-welcome-greeter) - antwortet auf Kommentatoren, die zum ersten Mal kommentieren.
- [Top Comment Pinner](#template-top-comment-pinner) - pinnt gehaltvolle Kommentare, sobald sie eine Abstimmungsschwelle überschreiten.
- [Thread Summarizer](#template-thread-summarizer) - veröffentlicht eine neutrale Zusammenfassung bei langen Threads.

Jede Vorlage öffnet ein vorausgefülltes Bearbeitungsformular mit bereits ausgewähltem **Status: Trockenlauf**.

### 3. Review and save

Im Bearbeitungsformular sollten Sie mindestens folgende Felder ausfüllen:

- **Internal name.** Ein kurzer Bezeichner, der in Administrations-Dashboards verwendet wird.
- **Display name.** Was öffentlich erscheint, wenn der Agent einen Kommentar postet.
- **Initial prompt.** Bearbeiten Sie das Prompt der Vorlage, damit es zu Ihrer Stimme und Ihren spezifischen Regeln passt.
- **Approvals.** Markieren Sie die Aktionen, die eine menschliche Überprüfung erfordern sollen, bevor sie ausgeführt werden. Wir empfehlen mindestens `ban_user` für jeden moderationsähnlichen Agenten. Siehe [Approval Workflow](#approval-workflow).

Klicken Sie auf **Save agent**.

### 4. Watch it in dry-run

Der Agent ist jetzt live im **Trockenlauf**. Er erhält seine Auslöser, ruft das Modell auf und protokolliert Aktionen auf der Seite [Run History](#run-history) – mit dem **Dry Run**-Abzeichen in jeder Zeile – trifft jedoch keine echten Maßnahmen. Rufen Sie einige der Ausführungsdetails auf (siehe [Run Detail View](#run-detail-view)) und sehen Sie sich an:

- Die Aktionen, die der Agent ausgewählt hat.
- Die Begründung und das Vertrauen zu jeder Aktion.
- Die vollständige LLM-Transkription.

Wenn der Agent Entscheidungen trifft, denen Sie nicht zustimmen, bearbeiten Sie das Initial Prompt oder markieren Sie mehr Approvals.

### 5. Run a test against past comments

Klicken Sie auf der Agents-Übersichtsseite in der Zeile des Agents auf **Test run**. Das Formular hat ein einzelnes numerisches Eingabefeld **Days** (1 bis 90). Stichprobengröße und das harte Limit für ausgewertete Kommentare werden informativ angezeigt – sie werden serverseitig berechnet und nicht vom Benutzer festgelegt. Die Wiedergabe läuft gegen historische Kommentare, ohne echte Aktionen vorzunehmen, und berichtet, was der Agent **getan hätte** im Vergleich zu dem, was tatsächlich passiert ist (wurde der Kommentar später genehmigt, als Spam markiert, gelöscht usw.). Siehe [Test Runs (Replays)](#test-runs-replays).

### 6. Flip to Enabled

Wenn Sie mit dem Trockenlauf und den Wiedergabe-Ergebnissen zufrieden sind, bearbeiten Sie den Agenten und ändern Sie den **Status** auf **Enabled**. Ab hier erfolgen echte Aktionen. Die Run History-Seite zeigt nun Live-Ausführungen ohne das Trockenlauf-Abzeichen, und jede Aktion, die Sie zur Genehmigung markiert haben, erscheint im [approvals inbox](#approval-workflow).

### What's next

- Legen Sie [Budgets](#budgets-overview) und [Budget Alerts](#budget-alerts) fest.
- Konfigurieren Sie [Webhooks](#webhooks-overview), wenn externe Systeme auf Agenten-Ereignisse reagieren sollen.
- Fügen Sie [Community Guidelines](#community-guidelines) hinzu, um die Entscheidungen des Agents mit Ihrer schriftlichen Richtlinie in Einklang zu bringen.