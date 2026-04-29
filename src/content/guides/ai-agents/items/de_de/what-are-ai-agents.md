Ein **KI-Agent** ist ein autonomer Arbeiter, der auf Ihren FastComments-Tenant beschränkt ist, auf Ereignisse in Ihrer Community achtet und in Ihrem Namen handelt.

Jeder Agent hat drei Dinge, die Sie kontrollieren:

1. **Eine Persönlichkeit.** Ein frei formulierter Anfangs-Prompt, der Ton, Rolle und Entscheidungsstil definiert („Sie sind ein herzlicher Community-Begrüßer“, „Sie setzen die Community-Regeln durch, neigen aber eher zu Verwarnungen als zu Sperrungen“ usw.).
2. **Einen oder mehrere Auslöser.** Eine Liste von Ereignissen, die den Agenten aufwecken – ein neuer Kommentar, ein Kommentar, der eine Schwelle bei Stimmen oder Meldungen überschreitet, eine Moderatorenaktion, der erste Kommentar eines Benutzers auf der Seite und andere. Die vollständige Liste finden Sie in [Trigger Events Overview](#triggers-overview).
3. **Eine Allowlist von Tools.** Was der Agent tun darf – einen Kommentar posten, abstimmen, anpinnen, sperren, als Spam markieren, einen Benutzer sperren, per DM verwarnen, ein Abzeichen vergeben, E-Mails senden, ein gemeinsames Gedächtnis speichern und durchsuchen. Die vollständige Liste finden Sie in [Allowed Tool Calls Overview](#tools-overview).

Wenn ein Auslöser ausgelöst wird, erhält der Agent eine Kontextnachricht, die beschreibt, was passiert ist (der Kommentar, die Seite, optionaler Thread-/Benutzer-/Seitenkontext), und wird mit seinem Anfangs-Prompt und Ihren Community-Richtlinien aufgefordert. Er ruft dann Tools auf, um zu handeln, und protokolliert bei jedem Aufruf eine Begründung und eine Vertrauensbewertung.

### Agenten laufen asynchron

Agenten blockieren **nie die Nutzeraktion, die sie ausgelöst hat**. Ein Leser postet einen Kommentar, der Kommentar wird gespeichert und im Thread angezeigt, die Antwort wird zurückgegeben, und erst *danach* läuft der Agent – entweder sofort oder nach einer konfigurierten Verzögerung (siehe [Deferred Triggers](#trigger-deferred-delay)). Nichts, was der Agent tut, fügt der nutzerseitigen Erfahrung Latenz hinzu.

### Warum man sie einsetzen sollte

- **Moderation in großem Maßstab.** Markieren Sie offensichtlichen Spam und sperren Sie wiederholte Störer, ohne die Queue rund um die Uhr überwachen zu müssen.
- **Begrüßen Sie neue Kommentierende.** Antworten Sie Erstkommentaren in Ihrer Stimme.
- **Heben Sie die besten Inhalte hervor.** Pinnen Sie substanzielle Top-Level-Kommentare, sobald sie eine Stimmen-Schwelle überschreiten.
- **Setzen Sie Ihre Richtlinien konsistent durch.** Wenden Sie denselben Richtlinientext auf jeden Grenzfall-Kommentar an.
- **Fassen Sie lange Threads zusammen.** Posten Sie neutrale Zusammenfassungen von Debatten über mehrere Seiten.

### Was Sie in Kontrolle hält

- **Trockenlaufmodus.** Jeder neue Agent wird standardmäßig im **Dry Run** ausgeliefert: Er verarbeitet Auslöser, führt das Modell aus und protokolliert, was er *tun würde*, führt jedoch keine realen Aktionen aus. Siehe [Dry-Run Mode](#dry-run-mode).
- **Genehmigungen.** Jede Teilmenge von Aktionen kann hinter einer menschlichen Genehmigungsgenehmigt werden. Siehe [Approval Workflow](#approval-workflow).
- **Pro-Agent- und pro-Konto-Budgets.** Harte tägliche und monatliche Obergrenzen. Siehe [Budgets Overview](#budgets-overview).
- **Tool-Allowlist.** Nicht erlaubte Tools werden aus der Palette des Modells entfernt – der Agent kann diese Tools buchstäblich nicht anfordern. Siehe [Allowed Tool Calls Overview](#tools-overview).
- **Prüffelder bei jeder Aktion.** Das Modell muss eine Begründung und eine Vertrauensbewertung angeben. Beides erscheint im Run-Zeitverlauf und bei jeder Genehmigung. Siehe [Run Detail View](#run-detail-view).
- **EU DSA Artikel 17.** In der EU-Region sind vollautomatische Sperrungen untersagt. Siehe [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Kein Training mit Ihren Daten.** FastComments verwendet Anbieter, die Ihre Prompts oder Kommentare nicht zum Training verwenden.

### Wie sie neben der menschlichen Moderation eingesetzt werden

Agenten und menschliche Moderatoren teilen sich dieselbe Kommentarplattform: Agenten führen Aktionen über dieselben Kanäle aus (approve, spam, ban, badge, pin, lock, write) und diese Aktionen erscheinen in denselben [Comment Logs](/guide-moderation.html#comment-logs), auf derselben [Moderation Page](/guide-moderation.html#moderate-comments-page) und in denselben Benachrichtigungsströmen. Agenten und Menschen sehen die Arbeit des jeweils anderen und können darauf reagieren – Moderatorenaktionen sind selbst gültige Agentenauslöser (siehe [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) und ähnliche).