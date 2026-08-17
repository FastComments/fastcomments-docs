FastComments setzt Artikel 17 des EU‑Digital Services Act für Mandanten in der EU‑Region durch: **vollautomatisierte Benutzersperrungen sind nicht erlaubt**.

### Was das in der Praxis bedeutet

Wenn Ihr Mandant sich in der EU‑Region befindet, im Agent‑Bearbeitungsformular:

- Das Kontrollkästchen **Approvals** für `ban_user` ist **aktiviert und gesperrt** und kann nicht abgewählt werden.
- Das Etikett lautet: "EU DSA Artikel 17: Benutzersperrungen erfordern eine menschliche Überprüfung. 'Einen Benutzer sperren' ist aktiviert und kann in der EU‑Region nicht vollautomatisiert werden."
- Ein Tooltip in der Genehmigungsspalte lautet: "Durch EU DSA Artikel 17 gesperrt – vollautomatisierte Sperrungen sind in der EU‑Region nicht erlaubt."

Egal, was Sie sonst noch konfigurieren, jeder `ban_user`‑Aufruf von einem beliebigen Agenten auf einem EU‑Region‑Mandanten wird an den [approvals inbox](#approval-workflow) zur menschlichen Überprüfung gesendet. Die Sperrung erfolgt erst, wenn ein Mensch sie genehmigt.

### Warum dies auf Plattform‑Ebene und nicht auf Prompt‑Ebene erzwungen wird

System‑Prompts können von einem ausreichend fehlverhaltenden Modell ignoriert oder umgangen werden. Die Einhaltung von Artikel 17 ist zu wichtig, um sich auf das gute Verhalten des Modells zu verlassen; sie muss ein harter serverseitiger Gate sein, den der Tool‑Dispatcher selbst durchsetzt. Genau das tun wir.

### Was durch Genehmigung geht und was nicht

- **`ban_user`**: immer in der EU gesperrt. Einschließlich:
  - Sichtbare Sperrungen (`shadowBan: false`).
  - Schatten‑Sperrungen (`shadowBan: true`).
  - Sperrungen mit `deleteAllUsersComments: true`.
  - Sperrungen mit `banIP: true`.
- Alle Sperrungsvarianten landen im Genehmigungs‑Posteingang mit der Begründung und dem Vertrauen des Agenten; ein Mensch genehmigt oder lehnt ab.

Die anderen Agent‑Werkzeuge (`mark_comment_spam`, `warn_user`, `lock_comment` usw.) sind **nicht** von Artikel 17 betroffen. Sie können weiterhin automatisiert werden. Artikel 17 bezieht sich speziell auf Benutzersperrungen.

### Was ist mit Nicht‑EU‑Mandanten

Die Sperre gilt nicht außerhalb der EU‑Region. Sie können `ban_user` trotzdem hinter einer Genehmigung sperren – wir empfehlen dies dringend für die ersten Wochen des Lebens eines Moderations‑Agents – aber sie wird nicht erzwungen.

### Schatten‑Sperrungen

Schatten‑Sperrungen zählen für die Zwecke von Artikel 17 als Sperrungen (der Benutzer kann posten, aber sein Inhalt wird verborgen). Sie werden identisch wie sichtbare Sperrungen gesperrt.

### Regions‑Erkennung

Die Region wird auf Prozessebene durch die Umgebungsvariable `REGION` in der FastComments‑Bereitstellung bestimmt (ausgelesen von `isEURegion()` in `models/constants.ts`). Es gibt kein regionsspezifisches Feld pro Mandant – die Sperre gilt für jeden Mandanten einer EU‑bereitgestellten Instanz. Wenn Sie Ihre Daten von einer Nicht‑EU‑Bereitstellung zu einer EU‑Bereitstellung migrieren, tritt die Sperre für alle Mandanten dieser Instanz in Kraft.

### Was, wenn alle Prüfer nicht verfügbar sind

Die Genehmigung verbleibt im Posteingang, bis sie entschieden wird. Sie verfällt automatisch 90 Tage nach Erstellung. Es gibt keinen Pfad "kein Prüfer verfügbar, automatisierte Entscheidung zulassen" – das würde den Sinn von Artikel 17 zunichte machen.

Wenn Ihre Community ein so hohes Volumen hat, dass EU‑Sperrungen nicht in angemessener Zeit geprüft werden können, sollten Sie in Betracht ziehen:

- Mehr Prüfer hinzuzufügen (siehe [Approval Notifications](#approval-notifications)).
- Den Agenten stärker auf die Nutzung von [`warn_user`](#tool-warn-user) umzustellen, da Warnungen nicht unter Artikel 17 fallen.
- Das Sperrungs‑Verlangen des Agenten zu reduzieren, indem Sie die [community guidelines](#community-guidelines) oder den [initial prompt](#personality-prompt) verschärfen.

### Siehe auch

- [Tool: ban_user](#tool-ban-user) für das, was `ban_user` tut, und die destruktiven Optionen hinter zusätzlichen Opt‑Ins.
- [Approval Workflow](#approval-workflow) für den vollständigen Genehmigungs‑Lebenszyklus.

---