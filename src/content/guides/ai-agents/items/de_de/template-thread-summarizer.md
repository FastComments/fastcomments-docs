**Vorlagen-ID:** `thread_summarizer`

Der Thread-Zusammenfasser veröffentlicht am Ende eines langen Threads eine neutrale, einabsätzige Zusammenfassung. Er verwendet eine 30-minütige Verzögerung, damit sich der Thread setzen kann, bevor der Agent ihn betrachtet.

Die eingebaute Prompt weist den Agenten an, nicht zu redigieren – das ist entscheidend. Ohne diese Anweisung neigt das Modell zu Formulierungen wie "meiner Meinung nach", die unter dem Anzeigenamen Ihres Kontos schlecht wirken.

### Auslöser

- **Neuer Kommentar gepostet** (`COMMENT_ADD`).
- **Auslöser-Verzögerung**: 30 Minuten (1800 Sekunden). Siehe [Verzögerte Trigger](#trigger-deferred-delay).

Die 30-minütige Verzögerung bedeutet, dass der Agent einmal ausgeführt wird, eine halbe Stunde nachdem ein Kommentar eingegangen ist, und den Thread so liest, wie er zu diesem Zeitpunkt aussieht. Es ist nicht "bei jeder Antwort zusammenfassen" – die Deferred-Trigger-Warteschlange fasst mehrere neue Kommentar-Ereignisse im selben Thread zusammen, dupliziert sie aber nicht über separate Fenster hinweg. Sie wollen wahrscheinlich **eine benutzerdefinierte Regel in Ihrer Prompt hinzufügen**, wie z. B. "keine neue Zusammenfassung posten, wenn der Agent diesen Thread in den letzten 24 Stunden bereits zusammengefasst hat", und sich auf den Kontext plus die [Speicher-Tools](#tools-overview) des Agenten verlassen, um das durchzusetzen.

### Zulässige Tools

- [`write_comment`](#tools-overview) - postet die Zusammenfassung selbst.
- [`pin_comment`](#tools-overview) - pinnt die Zusammenfassung, damit Leser sie oben im Thread sehen.
- [`unpin_comment`](#tools-overview) - entpinnt eine frühere Zusammenfassung desselben Agenten, bevor die neue gepinnt wird.

Der Zusammenfasser kann nicht moderieren oder mit Nutzern interagieren.

### Die Zusammenfassung pinnen

Der Agent postet einen neuen Kommentar mit `write_comment` und ruft dann `pin_comment` mit der zurückgegebenen Kommentar-ID auf. Bei nachfolgenden Ausführungen gegen denselben Thread weist die Prompt ihn an, zuerst `unpin_comment` für seine vorherige Zusammenfassung aufzurufen – die Plattform erzwingt selbst keine Regel für genau einen gepinnten Kommentar pro Thread, sodass das Belassen der vorherigen gepinnten Zusammenfassung zu zwei nebeneinander gepinnten Zusammenfassungen führt. Aktivieren Sie "Include parent comment and prior replies in the same thread" in den [Kontextoptionen](#context-options), damit der Agent die vorherige gepinnte Zusammenfassung sehen kann.

### Empfohlene Ergänzungen vor dem Live-Gang

- **Aktivieren Sie "Include parent comment and prior replies in the same thread"** in den [Kontextoptionen](#context-options). Ein Zusammenfasser ohne Thread-Kontext ist nutzlos.
- **Passen Sie die Regel für die minimale Thread-Größe an.** "Weniger als 5 Kommentare" ist die Standardeinstellung der Prompt, aber in aktiven Communities sind 10–20 angemessener. Bearbeiten Sie die Prompt direkt.
- **Beschränken Sie auf bestimmte URL-Muster**, wenn Sie Zusammenfassungen nur auf Long-Form-Seiten und nicht auf Ankündigungen oder Produktseiten möchten. Siehe [Umfang: URL- und Gebietsschema-Filter](#scope-url-locale).
- **Achten Sie auf die Kosten.** Zusammenfassungen sind die token-intensivste Vorlage, weil sie bei jeder Ausführung den ganzen Thread liest. Legen Sie vor dem Umschalten auf Aktiviert ein strenges [tägliches Budget](#budgets-overview) fest.

### Vermeidung wiederholter Zusammenfassungen

Der Agent hat Zugriff auf [`save_memory`](#tools-overview) und [`search_memory`](#tools-overview) – Sie können die Prompt erweitern, damit er angewiesen wird, Notizen wie "summarized {thread urlId}" zu speichern und vor dem erneuten Posten nach ihnen zu suchen. Der Speicher wird von allen Agenten in Ihrem Mandanten geteilt.