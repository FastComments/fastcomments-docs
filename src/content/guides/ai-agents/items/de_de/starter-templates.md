FastComments liefert vier Starter-Vorlagen, damit Sie keinen funktionsfähigen Agenten von Grund auf neu schreiben müssen. Sie sind über die [AI Agents-Seite](https://fastcomments.com/auth/my-account/ai-agents) erreichbar, indem Sie auf **Vorlagen durchsuchen** klicken.

Wenn Sie eine Vorlage auswählen:

1. Der Agent wird mit **Status: Dry Run** erstellt und erhält einen internen Namen basierend auf der Vorlage (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Falls dieser Name in Ihrem Mandanten bereits vergeben ist, wird eine numerische Endung angehängt.
2. Sie landen direkt im Bearbeitungsformular, in dem alles vorausgefüllt ist – Prompt, Triggers, zulässige Aktionen und ggf. Schwellenwerte. Ein Banner oben lautet "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Noch ist nichts aktiviert. Der Agent wird nicht handeln, bis Sie speichern und entweder Dry Run eingeschaltet lassen (zur Beobachtung) oder auf Enabled umstellen.

### Die vier Vorlagen

- **[Moderator](#template-moderator)** - überprüft neue und markierte Kommentare, ermahnt Ersttäter und eskaliert zu einem Bann erst nach einer Verwarnung. Löst bei neuen Kommentaren und bei flag-threshold crossings aus (Standard-Flag-Schwelle: 3). Zugelassene Tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - antwortet erstklassig und persönlich auf Erstkommentare von neuen Nutzern. Löst bei new-user-first-comment aus. Zugelassenes Tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pinnt gehaltvolle Top-Level-Kommentare, sobald sie einen Vote-Threshold überschreiten (Standard: 10), und entfernt zuvor gepinnte Kommentare zuerst. Löst bei vote-threshold crossings aus. Zugelassene Tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - postet nach einer Verzögerung eine neutrale, einabsätzige Zusammenfassung in langen Threads und pinnt diese dann. Löst bei neuen Kommentaren mit einer 30-minütigen Verzögerung aus, damit sich der Thread vor der Zusammenfassung beruhigt. Zugelassene Tools: `write_comment`, `pin_comment`, `unpin_comment`.

### Anpassung einer Vorlage

Vorlagen sind Ausgangspunkte, keine Verträge. Sie sollten:

- Den **Initial prompt** an die Stimme Ihrer Community anpassen.
- **Triggers** hinzufügen oder entfernen, damit der Agent in der gewünschten Häufigkeit ausgeführt wird.
- **Approvals** für jede sensible Aktion hinzufügen – wir empfehlen dringend, `ban_user` bei moderatorähnlichen Vorlagen hinter eine Genehmigung zu stellen.
- **Community guidelines** hinzufügen, damit der Agent Ihre schriftliche Richtlinie konsequent anwendet. Siehe [Community Guidelines](#community-guidelines).
- Pro Agent angemessene **Budgets** festlegen, entsprechend der erwarteten Trigger-Anzahl.

Die Vorlage ist nur ein Fahrzeug, das sinnvolle Voreinstellungen vorausfüllt; nach dem Speichern gehört der Agent Ihnen.