FastComments setzt Artikel 17 des EU-Digitaldienstegesetzes (DSA) für Mandanten in der EU-Region durch: vollautomatisierte Benutzersperrungen sind **nicht erlaubt**.

### Was das in der Praxis bedeutet

Wenn Ihr Mandant in der EU-Region ist, im Agent-Edit-Formular:

- Das **Approvals**-Kontrollkästchen für `ban_user` ist **fest aktiviert** und kann nicht deaktiviert werden.
- Das Label lautet: "EU DSA Article 17: user suspensions require human review. 'Benutzer sperren' ist aktiviert und darf in der EU-Region nicht vollständig automatisiert werden."
- Ein Tooltip in der Approval-Spalte lautet: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Unabhängig von Ihrer sonstigen Konfiguration geht jeder `ban_user`-Aufruf eines beliebigen Agenten für einen Mandanten in der EU-Region in den [approvals inbox](#approval-workflow) zur menschlichen Überprüfung. Die Sperrung erfolgt erst, nachdem ein Mensch sie genehmigt hat.

### Warum dies auf Plattformebene und nicht auf Prompt-Ebene durchgesetzt wird

System-Prompts können von einem ausreichend fehlverhaltenden Modell ignoriert oder umgangen werden. Die Einhaltung von Artikel 17 ist zu wichtig, um sich auf das wohlwollende Verhalten des Modells zu verlassen; es muss ein hartes serverseitiges Gate sein, das der Tool-Dispatcher selbst durchsetzt. Genau das tun wir.

### Was genehmigungspflichtig ist und was nicht

- **`ban_user`**: in der EU immer genehmigungspflichtig. Einschließlich:
  - Sichtbare Sperrungen (`shadowBan: false`).
  - Shadow-Bans (`shadowBan: true`).
  - Sperrungen mit `deleteAllUsersComments: true`.
  - Sperrungen mit `banIP: true`.
- Alle Varianten von Sperrungen landen mit der Begründung und Zuversicht des Agenten im Approvals-Posteingang; ein Mensch genehmigt oder lehnt ab.

Die anderen Agent-Tools (`mark_comment_spam`, `warn_user`, `lock_comment` usw.) sind von Artikel 17 **nicht** betroffen. Sie können diese weiterhin automatisieren. Artikel 17 bezieht sich speziell auf Benutzersperrungen.

### Was ist mit Nicht-EU-Mandanten

Die Sperre gilt außerhalb der EU-Region nicht. Sie können `ban_user` dennoch hinter einer Genehmigungspflicht platzieren — wir empfehlen dies nachdrücklich in den ersten Wochen des Betriebs eines Moderationsagenten — aber es wird nicht erzwungen.

### Shadow-Bans

Shadow-Bans gelten für die Zwecke von Artikel 17 als Sperrungen (der Nutzer kann posten, aber seine Inhalte sind verborgen). Sie unterliegen der gleichen Genehmigungsregelung wie sichtbare Sperrungen.

### Regionserkennung

Die Region wird auf Prozessebene durch die Umgebungsvariable `REGION` in der FastComments-Installation bestimmt (ausgelesen von `isEURegion()` in `models/constants.ts`). Es gibt kein regionsbezogenes Feld pro Mandant — die Sperre gilt für jeden Mandanten auf einer in der EU bereitgestellten Instanz. Wenn Sie Ihre Daten von einer Nicht-EU-Installation auf eine EU-Installation migrieren, tritt die Sperre für alle Mandanten dieser Instanz in Kraft.

### Was passiert, wenn alle Prüfer nicht verfügbar sind

Die Genehmigungsanfrage verbleibt im Posteingang, bis eine Entscheidung getroffen wird. Sie läuft 90 Tage nach Erstellung automatisch ab. Es gibt keinen Weg wie "kein Prüfer verfügbar, automatisch entscheiden" — das würde den Zweck von Artikel 17 zunichte machen.

Wenn Ihre Community so viel Verkehr hat, dass EU-Sperrungen nicht in angemessener Zeit geprüft werden können, sollten Sie in Erwägung ziehen:

- Weitere Prüfer hinzuzufügen (siehe [Benachrichtigungen für Genehmigungen](#approval-notifications)).
- Den Agenten so zu konfigurieren, dass er `warn_user` aggressiver einsetzt, da Warnungen nicht unter Artikel 17 fallen.
- Die Bereitschaft des Agenten zu sperren zu senken, indem Sie die [Community-Richtlinien](#community-guidelines) oder das [Initial-Prompt](#personality-prompt) verschärfen.

### Siehe auch

- [Tool: ban_user](#tool-ban-user) für Informationen darüber, was `ban_user` bewirkt und welche destruktiven Optionen hinter zusätzlichen Opt-ins stehen.
- [Genehmigungs-Workflow](#approval-workflow) für den vollständigen Genehmigungslebenszyklus.