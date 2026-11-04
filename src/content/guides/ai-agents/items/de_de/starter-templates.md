FastComments liefert fünf Starter-Vorlagen mit, damit Sie keinen funktionierenden Agenten von Grund auf schreiben müssen. Sie sind über die [AI Agents-Seite](https://fastcomments.com/auth/my-account/ai-agents) erreichbar, indem Sie auf **Browse templates** klicken.

Wenn Sie eine Vorlage auswählen:

1. Der Agent wird mit **Status: Dry Run** und einem internen Namen basierend auf der Vorlage erstellt (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Falls dieser Name auf Ihrem Tenant bereits vergeben ist, wird ein numerischer Suffix hinzugefügt.
2. Sie landen direkt auf dem Bearbeitungsformular, in dem alles vorausgefüllt ist - Prompt, Trigger, erlaubte Aktionen und alle Schwellenwerte. Ein Banner am oberen Rand lautet: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Noch ist nichts aktiviert. Der Agent wird erst aktiv, wenn Sie speichern und entweder den Dry-Run-Modus beibehalten (zum Beobachten) oder auf Enabled umschalten.

### Die fünf Vorlagen

- **[Moderator](#template-moderator)** - prüft neue und gemeldete Kommentare, warnt Ersttäter und eskaliert erst nach einer Warnung zu einer Sperre. Wird bei neuen Kommentaren und beim Überschreiten des Meldeschwellenwerts ausgelöst (Standard-Meldeschwelle: 3). Erlaubte Tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - antwortet Erstkommentierern herzlich mit einer kurzen, persönlichen Begrüßung. Wird beim ersten Kommentar eines neuen Benutzers ausgelöst. Erlaubtes Tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - pinnt substanzielle Kommentare auf oberster Ebene, sobald sie einen Stimmschwellenwert überschreiten (Standard: 10), und hebt zuvor die Pinning des vorherigen Kommentars auf. Wird beim Überschreiten von Stimmschwellenwerten ausgelöst. Erlaubte Tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - veröffentlicht nach einer Verzögerung eine neutrale, ein Absatz lange Zusammenfassung in langen Threads und pinnt diese anschließend. Wird bei neuen Kommentaren mit einer Verzögerung von 30 Minuten ausgelöst, damit sich der Thread vor der Zusammenfassung beruhigt. Erlaubte Tools: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - überwacht Kommentarbearbeitungen auf Änderungen während des Threads, die Antworten verzerren, stellt den ursprünglichen Text wieder her und sendet dem Autor eine DM. Wird bei Kommentarbearbeitungen ausgelöst. Erlaubte Tools: `edit_comment`, `warn_user`, `send_dm`.

### Anpassen einer Vorlage

Vorlagen sind Ausgangspunkte, keine Verträge. Es wird erwartet, dass Sie:

- Den **Initial prompt** an die Stimme Ihrer Community anpassen.
- **Triggers** hinzufügen oder entfernen, um festzulegen, wie oft der Agent laufen soll.
- **Approvals** für jede sensible Aktion hinzufügen - wir empfehlen dringend, `ban_user` bei Moderator-artigen Vorlagen hinter eine Genehmigung zu stellen.
- **Community guidelines** hinzufügen, damit der Agent Ihre schriftliche Richtlinie konsistent anwendet. Siehe [Community Guidelines](#community-guidelines).
- Pro Agent **Budgets** festlegen, die zur erwarteten Anzahl an Triggern passen.

Die Vorlage ist nur ein Vehikel, das sinnvolle Standardwerte vorausfüllt; nach dem Speichern gehört der Agent Ihnen.