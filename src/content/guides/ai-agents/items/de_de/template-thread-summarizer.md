**Vorlagen-ID:** `thread_summarizer`

Der Thread-Zusammenfasser veröffentlicht am Ende eines langen Threads eine neutrale, einabsätzige Zusammenfassung. Er verwendet eine 30-minütige Verzögerung, damit sich der Thread beruhigen kann, bevor der Agent ihn betrachtet.

### Eingebautes Anfangsprompt

[inline-code-attrs-start title = 'Anfangsprompt der Thread-Zusammenfasser-Vorlage'; type='text' inline-code-attrs-end]
[inline-code-start]
Sie veröffentlichen neutrale Thread-Zusammenfassungen. Fassen Sie keine Threads zusammen, die weniger als 5 Kommentare haben. Bei längeren Threads fassen Sie die Hauptpositionen, Meinungsverschiedenheiten und offenen Fragen in einem kurzen Absatz zusammen. Nehmen Sie keine Partei und geben Sie keine wertenden Kommentare ab. Nachdem Sie die Zusammenfassung gepostet haben, pinnen Sie sie. Wenn eine frühere von Ihnen verfasste Zusammenfassung in diesem Thread bereits angepinnt ist, entpinnen Sie sie, bevor Sie die neue anpinnen.
[inline-code-end]

Die Anweisung „keine Wertungen abgeben“ ist maßgeblich. Ohne sie neigt das Modell zu Formulierungen wie „meiner Ansicht nach“, die unter dem Anzeigenamen Ihres Accounts schlecht wirken.

### Auslöser

- **Neuer Kommentar gepostet** (`COMMENT_ADD`).
- **Auslöseverzögerung**: 30 Minuten (1800 Sekunden). Siehe [Aufgeschobene Auslöser](#trigger-deferred-delay).

Die 30-minütige Verzögerung bedeutet, dass der Agent einmal ausgeführt wird, eine halbe Stunde nachdem ein Kommentar eingegangen ist, basierend auf dem Zustand des Threads in diesem Moment. Es ist nicht „bei jeder Antwort zusammenfassen“ – die Warteschlange für verzögerte Auslöser fasst mehrere neue Kommentar-Ereignisse desselben Threads zusammen, de-dupliziert sie aber nicht über separate Zeitfenster hinweg. Sie werden wahrscheinlich möchten, **eine benutzerdefinierte Regel in Ihrem Prompt hinzuzufügen**, wie z. B. „poste keine neue Zusammenfassung, wenn der Agent diesen Thread in den letzten 24 Stunden bereits zusammengefasst hat“, und sich auf den Kontext plus die Agenten-[Gedächtnis-Tools](#tools-overview) verlassen, um dies durchzusetzen.

### Zulässige Tools

- [`write_comment`](#tools-overview) - postet die Zusammenfassung selbst.
- [`pin_comment`](#tools-overview) - pinnt die Zusammenfassung, damit Leser sie oben im Thread sehen.
- [`unpin_comment`](#tools-overview) - entpinned eine frühere Zusammenfassung desselben Agenten, bevor die neue angepinnt wird.

Der Zusammenfasser kann nicht moderieren oder mit Nutzern interagieren.

### Die Zusammenfassung anpinnen

Der Agent postet einen neuen Kommentar mit `write_comment` und ruft dann `pin_comment` mit der zurückgegebenen Kommentar-ID auf. Bei nachfolgenden Durchläufen desselben Threads weist das Prompt ihn an, zuerst `unpin_comment` für seine vorherige Zusammenfassung aufzurufen – die Plattform selbst erzwingt keine Regel von nur einem angepinnten Kommentar pro Thread, sodass das Belassen der vorherigen Zusammenfassung angepinnt dazu führt, dass zwei angepinnte Zusammenfassungen nebeneinander erscheinen. Aktivieren Sie "Include parent comment and prior replies in the same thread" in den [Context Options](#context-options), damit der Agent die zuvor angepinnte Zusammenfassung sehen kann.

### Empfohlene Ergänzungen vor dem Livegang

- **Aktivieren Sie "Include parent comment and prior replies in the same thread"** in den [Context Options](#context-options). Ein Zusammenfasser ohne Thread-Kontext ist nutzlos.
- **Passen Sie die Regel zur minimalen Thread-Größe an.** "Weniger als 5 Kommentare" ist die Standardeinstellung des Prompts, aber in aktiven Communities sind 10–20 angemessener. Bearbeiten Sie das Prompt direkt.
- **Einschränken auf spezifische URL-Muster** wenn Sie Zusammenfassungen nur auf Longform-Seiten und nicht auf Ankündigungs- oder Produktseiten wünschen. Siehe [Bereich: URL- und Lokalisierungsfilter](#scope-url-locale).
- **Achten Sie auf Kosten.** Zusammenfassungen sind die tokenintensivste Vorlage, da sie bei jedem Lauf den gesamten Thread liest. Legen Sie ein striktes [tägliches Budget](#budgets-overview) fest, bevor Sie auf 'Enabled' umschalten.

### Vermeidung wiederholter Zusammenfassungen

Der Agent hat Zugriff auf [`save_memory`](#tools-overview) und [`search_memory`](#tools-overview) – Sie können das Prompt erweitern, um es anzuweisen, Notizen wie "summarized {thread urlId}" zu speichern und vor erneutem Posten danach zu suchen. Der Speicher (Memory) wird über alle Agenten in Ihrem Mandanten geteilt.

---