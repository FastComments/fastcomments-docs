Das Sperrwerkzeug ist die folgenreichste Aktion, die ein Agent ausführen kann. Es sperrt einen Benutzer aus Ihrer Community für eine festgelegte Dauer und mit einigen Optionen.

### Was es macht

Der Agent wählt eine von sechs Laufzeiten:

- Eine Stunde
- Ein Tag
- Eine Woche
- Ein Monat
- Sechs Monate
- Ein Jahr

Er wählt außerdem zwischen einer **sichtbaren Sperrung** (der Benutzer sieht eine klare Sperrmeldung und kann Einspruch einlegen) und einer **Schattenbann** (der Benutzer kann weiterhin posten, aber seine Inhalte sind für andere Benutzer verborgen). Die Anweisungen der Plattform sagen dem Agenten, sichtbare Sperrungen bei Erst- oder Grenzfällen zu bevorzugen und Schattenbanns bei eindeutig bösartigen Wiederholungstätern.

### Die beiden destruktiven Unteroptionen

Zwei zusätzliche Optionen sind **standardmäßig vor dem Agenten verborgen**. Um eine davon zu aktivieren, setzen Sie das entsprechende Kontrollkästchen im Abschnitt **Sperroptionen** im Bearbeitungsformular des Agenten:

- **Allow deleting all of the user's comments.** Wenn aktiviert, kann der Agent zusätzlich entscheiden, jeden Kommentar zu löschen, den der gesperrte Benutzer jemals in Ihrem Tenant gepostet hat. Nur für eindeutigen Spam, Doxxing oder koordinierte Missbrauchsfälle reservieren, bei denen der vorhandene Inhalt keinen Wert hat. **Zerstörerisch und irreversibel.**
- **Allow banning by IP.** Wenn aktiviert, kann der Agent zusätzlich entscheiden, die IP zu sperren, von der der Kommentar gepostet wurde. Nützlich gegen Umgehung von Sperren durch Alternativkonten. **Bei geteilten IPs vermeiden** (Firmen-, Schul- oder Mobilfunknetze) — unschuldige Benutzer im selben Netzwerk werden gesperrt.

Die Plattform erzwingt diese Beschränkungen auch serverseitig: selbst wenn der Agent abtrünnig wird und versucht, die Option zu nutzen, wird die Anfrage abgelehnt, sofern Sie nicht zugestimmt haben.

### Eskalationsrichtlinie

Bevor eine Sperre ausgesprochen wird, weist die Plattform den Agenten an:

1. Im [Agentenspeicher](#agent-memory-system) nach früheren Verwarnungen oder Notizen zum Benutzer zu suchen.
2. Eine [Verwarnung](#tool-warn-user) des Benutzers einer Sperre bei Erstverstößen vorzuziehen.
3. Den Warnschritt nur bei eindeutig schwerwiegenden Fällen (illegale Inhalte, Doxxing, koordinierter Spam) zu überspringen — und dies in seiner Begründung zu erläutern.

Diese Richtlinie ist Teil der Anweisungen an den Agenten, keine harte serverseitige Regel, weshalb **es dringend empfohlen wird, Sperrungen der Genehmigung zu unterstellen**.

### EU-Region: menschliche Genehmigung erforderlich

In der EU-Region ist dieses Tool nach Artikel 17 des Digital Services Act **für die Genehmigung gesperrt**. Jede Sperre eines Agenten auf einem Tenant in der EU-Region landet im [Genehmigungs-Posteingang](#approval-workflow) zur manuellen Prüfung. Siehe [EU DSA Artikel 17 Konformität](#eu-dsa-compliance).

### Empfehlungen

- Sperrungen überall – mindestens während des ersten Monats – der Genehmigung unterstellen.
- Schränken Sie die Option **delete-all-comments** immer durch Genehmigung ein, wenn Sie sie aktivieren — sie ist irreversibel.
- Erwägen Sie, die Option **IP ban** auch nachdem der Agent Vertrauen gewonnen hat weiterhin der Genehmigung zu unterstellen — die Auswirkungen einer IP-Sperre in einem geteilten Netzwerk zeigen sich nicht in der Ausführungshistorie des Agenten.

### Siehe auch

- [Benutzer sperren](/guide-moderation.html#banning-users) und [Benutzer sperren mit Wildcards](/guide-moderation.html#banning-users-wildcards) im Moderationsleitfaden, um zu erfahren, wie Sperren plattformweit funktionieren.
- [Benutzer verwarnen](#tool-warn-user) - der sanftere Eskalationsschritt.