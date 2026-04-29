---
Das Feld **Community guidelines** im Bearbeitungsformular ist ein optionaler Richtlinientextblock, der in die Kontextnachricht mit Benutzerrolle bei jedem Lauf für diesen Agenten aufgenommen wird. Er ist als nicht vertrauenswürdiger Text abgegrenzt (die gleiche Abgrenzung, die die Plattform auf Kommentartexte und andere von Nutzern bereitgestellte Inhalte anwendet), sodass das Modell ihn als Richtlinienreferenz und nicht als Systemanweisung behandelt. Es ist der kanonische Ort, um niederzuschreiben "welches Verhalten auf dieser Seite erlaubt ist und welches nicht", damit der Agent es konsistent anwendet.

### Worin es sich vom Initial-Prompt unterscheidet

- **Initial-Prompt** - die Rolle des Agenten und dessen Entscheidungsstil. "Du bist ein Moderator. Bevorzuge Verwarnungen gegenüber Sperrungen."
- **Community guidelines** - die Regeln deiner Community, in Sprache einer Richtlinie. "Keine persönlichen Angriffe. Keine werblichen Links von Accounts, die jünger als 24 Stunden sind. Off-Topic-Kommentare können entfernt werden, wenn ein Thread aufgeheizt ist."

Beide fließen in dasselbe Kontextfenster, aber sie gelangen auf verschiedenen Ebenen hinein - der Initial-Prompt ist Teil der Systemrolle, das Richtliniendokument ist als abgegrenzter Text in der Kontextnachricht mit Benutzerrolle enthalten. Die Trennung erleichtert das Bearbeiten, wenn du eines ändern möchtest, ohne das andere erneut lesen zu müssen.

### Was ein gutes Richtliniendokument ist

Ein kurzes, konkretes, von einer Person verfasstes Dokument. Aufzählungen funktionieren besser als Fließtext:

[inline-code-attrs-start title = 'Beispiel für Community-Richtlinien'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

Der Agent wendet dies bei jedem Lauf an. Wenn du die Richtlinien änderst, tritt die Änderung beim nächsten Auslöser in Kraft - frühere Läufe werden nicht rückwirkend neu bewertet.

### Was man hier nicht ablegen sollte

- **Ausgabeformatierungsanweisungen** ("antworte in HTML", "verwende Emojis"). Diese gehören in den [Initial-Prompt](#personality-prompt).
- **Lokalisierter Text.** Das Richtliniendokument ist, wie der Prompt, **nur auf Englisch** aus demselben Grund - maschinelle Übersetzung kann das Verhalten des Agenten stillschweigend verändern. Wenn du Richtlinien hast, die je nach Gebietsschema variieren, schreibe sie alle auf Englisch in dieses eine Dokument und strukturiere das Dokument z. B. als "für deutschsprachige Seiten: ..."
- **Lange Zitate externer Richtlinien.** Paraphrasiere. Langer Kontext kostet bei jedem Lauf Tokens.
- **PII oder Geheimnisse.** Dieser Text wird bei jedem Lauf an den LLM-Anbieter gesendet.

### Länge

Das Feld ist auf **4000 Zeichen** begrenzt (sowohl durch das Formular als auch durch die Save-Route erzwungen). Die Token-Kosten bei jedem Lauf sind proportional zur Länge, daher reichen in der Regel auch innerhalb des Limits ein paar hundert Wörter. Wenn deine Community-Richtlinien sich über viele Seiten erstrecken, fasse die Teile zusammen, die der Agent benötigt, und halte sie speziell für dieses Feld bereit.

### Versionierung

Es gibt keine eingebaute Versionshistorie für das Richtliniendokument - der zuletzt gespeicherte Wert ist das, was der Agent verwendet. Wenn du eine Historie möchtest, kopiere das Dokument vor jeder größeren Änderung in dein eigenes Nachverfolgungssystem. Der [Prompts verfeinern](#refining-prompts) Flow kann Änderungen am *Initial-Prompt* aufzeichnen, versioniert das Richtliniendokument jedoch nicht.

---