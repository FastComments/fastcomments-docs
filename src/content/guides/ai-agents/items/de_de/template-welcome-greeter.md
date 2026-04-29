**Vorlagen-ID:** `welcome_greeter`

Der Welcome Greeter antwortet freundlich auf Erstkommentatoren. Es ist die risikofreundlichste Vorlage (keine destruktiven Werkzeuge) und ein guter erster Agent, der live geschaltet werden kann.

### Auslöser

- **Ein neuer Benutzer veröffentlicht seinen ersten Kommentar auf dieser Website** (`NEW_USER_FIRST_COMMENT`).

Dieses Ereignis wird genau einmal pro Benutzer ausgelöst, sodass der Agent nicht in eine Schleife geraten kann. Siehe [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Zugelassene Tools

- [`write_comment`](#tools-overview)

Das ist das einzige Tool – der Agent kann wortwörtlich nicht moderieren, abstimmen, sperren oder Direktnachrichten senden (DM).

### Empfohlene Ergänzungen vor dem Livegang

- **Setzen Sie den Anzeigenamen** auf etwas Einladendes – "Community Bot", Ihr Seitenmaskottchen oder Ihr Markenname. Der Anzeigename ist das, was Leser an der Willkommensantwort sehen.
- **Aktivieren Sie "Seiten-Titel, Untertitel, Beschreibung und Meta-Tags einbeziehen"** in den [Kontextoptionen](#context-options). Die Antworten des Greeters werden deutlich besser, wenn er auf den tatsächlichen Inhalt der Seite Bezug nehmen kann.
- **Berücksichtigen Sie Lokalisierungsbeschränkungen**, wenn Sie in mehreren Sprachen operieren. Eine Willkommensantwort in der falschen Sprache ist störender als eine fehlende Antwort. Siehe [Bereich: URL- und Lokalisierungsfilter](#scope-url-locale).

### Warum keine Genehmigungen erforderlich sind

Der Agent schreibt nur neue Kommentare und nur bei einem einmaligen Auslöser. Im schlimmsten Fall: eine unbeholfene Begrüßung. Es gibt keine destruktive Aktion, die eingeschränkt werden müsste. Die meisten Betreiber setzen diesen Agenten ohne jegliche Genehmigungen ein, sobald der Trockenlauf sauber aussieht.