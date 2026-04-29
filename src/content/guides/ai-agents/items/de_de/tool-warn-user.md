Das Warn-Tool sendet eine private Direktnachricht (DM), die einen Nutzer über einen bestimmten Kommentar warnt, und zeichnet gleichzeitig die Verwarnung im gemeinsamen [Agentenspeicher](#agent-memory-system) auf. Die beiden Schreibvorgänge sind atomar - der Nutzer sieht niemals eine Warnung, die nicht auch protokolliert ist.

### Warum es existiert

Die Eskalationsrichtlinie der Plattform lautet **zuerst verwarnen, nur bei erneuter Verfehlung sperren**. Das Warn-Tool macht diese Richtlinie praktisch anwendbar: Es gibt dem Nutzer die Möglichkeit, sein Verhalten zu korrigieren, und der Verwarnungseintrag ist das, was ein zukünftiger Agent findet, wenn er vor einer möglichen Sperre den Speicher durchsucht.

Das Tool vermeidet außerdem Duplikate: Wenn der Agent dem gleichen Nutzer bereits wegen desselben Kommentars eine Verwarnung erteilt hat, hat eine zweite Verwarnung keine Wirkung. Ein LLM, das in einer Schleife läuft oder denselben Kommentar erneut verarbeitet, kann den Nutzer so nicht mit mehreren Verwarnungen zuspammen.

### Was in die Verwarnung gehört

Eine kurze Nachricht (auf 1000 Zeichen begrenzt), die dem Nutzer als DM angezeigt wird. Wirksame Verwarnungen sind:

- **Spezifisch** - "Persönliche Angriffe gegen namentlich genannte Nutzer sind in dieser Community nicht erlaubt" ist besser als "Ihr Kommentar wurde gemeldet."
- **Kurz** - höchstens ein paar Sätze.
- **Handlungsorientiert** - sagen Sie dem Nutzer, was er ändern soll. "Bitte bearbeiten Sie Ihren Kommentar und entfernen Sie den genannten Nutzer, sonst wird er entfernt."

Sie schreiben die Nachricht nicht selbst; der Agent tut das auf Grundlage der [initialen Aufforderung](#personality-prompt) und der [Community-Richtlinien](#community-guidelines). Ihre Aufgabe ist es, eine Aufforderung zu verfassen, die gute Verwarnungen erzeugt.

### Wann zulassen

Für jeden Moderations‑Agenten. Die Moderator‑Vorlage aktiviert es standardmäßig.

### Genehmigungen

Wird seltener durch Genehmigungen geregelt als [Benutzer sperren](#tool-ban-user). Es lohnt sich, in den ersten Wochen der Laufzeit eines Agents die Genehmigungspflicht einzuschalten, damit Sie schlechte Verwarnungen erkennen, bevor sie verschickt werden; die meisten Betreiber deaktivieren das Gate jedoch, sobald der Agent zuverlässige Ergebnisse liefert.

### Siehe auch

- [Benutzer sperren](#tool-ban-user) - der nächste Eskalationsschritt.
- [Agentenspeicher](#agent-memory-system) - wo Verwarnungsaufzeichnungen gespeichert sind.