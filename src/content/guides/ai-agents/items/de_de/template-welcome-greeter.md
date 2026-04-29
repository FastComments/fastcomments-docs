**Vorlagen-ID:** `welcome_greeter`

Der Welcome Greeter begrüßt Erstkommentierende herzlich. Er ist die risikoärmste Vorlage (keine destruktiven Werkzeuge) und ein guter erster Agent für den Live-Einsatz.

### Eingebaute Anfangsaufforderung

[inline-code-attrs-start title = 'Initiale Aufforderung des Welcome Greeter-Templates'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Auslöser

- **Ein neuer Nutzer veröffentlicht hier seinen ersten Kommentar** (`NEW_USER_FIRST_COMMENT`).

Dieses Ereignis wird pro Nutzer genau einmal ausgelöst, sodass der Agent nicht in eine Schleife geraten kann. Siehe [Trigger: Neuer Nutzer — erster Kommentar](#trigger-new-user-first-comment).

### Erlaubte Werkzeuge

- [`write_comment`](#tools-overview)

Das ist das einzige Werkzeug — der Agent kann buchstäblich nicht moderieren, abstimmen, sperren oder Direktnachrichten (DM) senden.

### Empfohlene Ergänzungen vor dem Live-Betrieb

- **Legen Sie den Anzeigenamen fest** auf etwas Einladendes — "Community Bot", Ihr Seitenmaskottchen oder Ihr Markenname. Der Anzeigename ist das, was Leser an der Begrüßungsantwort sehen.
- **Aktivieren Sie "Seitentitel, Untertitel, Beschreibung und Meta-Tags einbeziehen"** in den [Kontextoptionen](#context-options). Die Antworten des Greeters werden deutlich besser, wenn er auf den tatsächlichen Seiteninhalt Bezug nehmen kann.
- **Berücksichtigen Sie Lokalisierungsbeschränkungen**, wenn Sie in mehreren Sprachen tätig sind. Eine Begrüßungsantwort in der falschen Sprache ist störender als eine fehlende Antwort. Siehe [Geltungsbereich: URL- und Lokalisierungsfilter](#scope-url-locale).

### Warum keine Genehmigungen erforderlich sind

Der Agent schreibt nur neue Kommentare und reagiert nur auf einen einmaligen Auslöser. Im schlimmsten Fall entsteht eine unbeholfene Begrüßung. Es gibt keine destruktive Aktion, die genehmigt werden müsste. Die meisten Betreiber setzen diesen Agenten ohne jegliche Genehmigungen ein, sobald der Trockenlauf sauber aussieht.

---