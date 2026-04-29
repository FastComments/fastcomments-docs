Die **Tools** eines Agenten sind die Aktionen, die er ausführen kann. Das Agenten-Bearbeitungsformular hat einen Abschnitt **Allowed tool calls**, in dem Sie die Tools ankreuzen, die dieser Agent verwenden darf, und einen Abschnitt **Approvals**, in dem Sie die Aktionen ankreuzen, die ein Mensch genehmigen muss, bevor sie wirksam werden.

Es gibt drei Stufen für jedes Tool:

- **Disallowed** - der Agent kann es nicht sehen oder verwenden.
- **Allowed, no approval** - der Agent verwendet es direkt. Wird in der Ausführungshistorie aufgezeichnet.
- **Allowed, with approval** - der Aufruf des Agenten wird zur menschlichen Überprüfung in die Warteschlange gestellt und läuft erst, wenn ein Mensch zustimmt.

Nicht erlaubte Tools sind stumm: der Agent kann sie nicht anfordern und die Plattform lehnt sie vollständig ab. Tools, die genehmigungspflichtig sind, werden immer über den [approvals inbox](#approval-workflow) geleitet.

### Prüfpfad für jede Aktion

Jede Aktion des Agenten wird mit einer kurzen Begründung (1–2 Sätze, die erklären, warum) und einer Vertrauenswürdigkeitsskala (0.0–1.0) aufgezeichnet. Beides erscheint in der [Run Detail View](#run-detail-view) und bei jeder [approval](#approval-workflow). Die Suche im Speicher ist die einzige schreibgeschützte Ausnahme: Sie wird nicht als Aktion aufgezeichnet und ist unabhängig von der Allowlist immer verfügbar.

### Tool-Referenz

#### Kommentare posten

Ermöglicht dem Agenten, einen Kommentar unter eigenem Namen zu posten. Der Kommentar wird öffentlich unter dem Anzeigenamen des Agenten angezeigt. Wird von Begrüßungs- und Zusammenfassungsagenten verwendet. Umkehrbar – jeder Moderator kann einen schlechten Kommentar entfernen. Gewöhnlich ohne Genehmigung erlaubt; setzen Sie eine Genehmigungspflicht, wenn jede öffentlich sichtbare Nachricht Ihrer Community von einem Menschen geprüft werden soll.

#### Kommentare bewerten

Ermöglicht dem Agenten, einen Kommentar hoch- oder runterzuvoten. Die Stimme zählt wie jede andere Stimme zur Gesamtbewertung des Kommentars. Die meisten Communities bevorzugen, dass Bots nicht abstimmen; in keiner Starter-Vorlage aktiviert. Wenn Sie es erlauben, ist die Abstimmung umkehrbar.

#### Kommentar anpinnen / entpinnen

Ermöglicht dem Agenten, einen Kommentar oben auf der Seite anzupinnen oder einen bereits angepinnten Kommentar zu entpinnen. Die Plattform erzwingt keine Regel „ein Pin pro Thread“, daher sollte ein anpinnender Agent angewiesen werden, zuerst den vorherigen angepinnten Kommentar zu entpinnen. Wird von der Top Comment Pinner-Vorlage verwendet. Umkehrbar; normalerweise ohne Genehmigung erlaubt.

#### Kommentar sperren / entsperren

Ermöglicht dem Agenten, weitere Antworten unter einem Kommentar zu verhindern oder Antworten wiederherzustellen. Der gesperrte Kommentar bleibt sichtbar. Nützlich für Abkühlphasen bei hitzigen Threads, kombiniert mit einer verzögerten Entsperrung. Umkehrbar, aber für Ihre Community sichtbar; erwägen Sie, es in risikoreichen Communities hinter eine Genehmigungspflicht zu stellen.

#### Als Spam markieren / Spam-Markierung entfernen

Ermöglicht dem Agenten, einen Kommentar als Spam zu markieren (vor Lesern verbergen und den Spam-Klassifizierer damit füttern) oder dieses Flag zu entfernen. Das grundlegende Tool für jeden Moderationsagenten. Umkehrbar. Ziehen Sie in Erwägung, es in den ersten Wochen hinter eine Genehmigungspflicht zu stellen, während Sie Vertrauen in den Agenten aufbauen.

#### Kommentar freigeben / Freigabe aufheben

Ermöglicht dem Agenten, einen zurückgehaltenen Kommentar für Leser sichtbar zu machen oder einen bereits sichtbaren Kommentar zu verbergen. Am nützlichsten bei tenants, die neue Kommentare zur Moderatorenprüfung zurückhalten. Hohe Risiken beim Aufheben der Freigabe eines sichtbaren Kommentars – erwägen Sie, dies hinter eine Genehmigungspflicht zu stellen.

#### Einen Kommentar als geprüft markieren

Ein Queue-Status-Tool: markiert einen Kommentar als „ein Moderator (oder Agent) hat dies angesehen“. Ändert nicht die Sichtbarkeit. Geringes Risiko; selten hinter Genehmigungspflicht.

#### Abzeichen vergeben

Ermöglicht dem Agenten, einem Nutzer ein Abzeichen aus der Abzeichenkonfiguration Ihres tenants zu vergeben. Rückgängig durch einen Moderator. Selten hinter Genehmigungspflicht. Der Agent muss die Abzeichen-ID kennen, daher sollten Sie die relevanten IDs in Ihren [community guidelines](#community-guidelines) oder im [initial prompt](#personality-prompt) aufführen.

#### E-Mail senden

Ermöglicht dem Agenten, eine Nur-Text-E-Mail von `noreply@fastcomments.com` an eine von ihm gewählte Adresse zu senden. Sparsam einsetzen – E-Mails sind das aufwändigste Tool und fehlerhafte E-Mails sind schwer rückgängig zu machen. Erwägen Sie dringend, es hinter eine Genehmigungspflicht zu stellen, und leiten Sie Genehmigungs-E-Mails an die Person weiter, die das Postfach verwaltet, an das der Agent schließlich senden wird.

#### Agenten-Speicher speichern / durchsuchen

Zwei gekoppelte Tools, die einen gemeinsamen Notizpool über den Nutzer lesen und schreiben, für den ein Trigger ausgelöst wurde. Der Speicher wird unter allen Agenten in Ihrem tenant geteilt, sodass die Notizen eines Triage-Agenten die Entscheidungen eines Moderationsagenten beeinflussen. Die Suche ist schreibgeschützt und immer verfügbar; das Speichern ist selten hinter Genehmigungspflicht. Siehe [Agent Memory System](#agent-memory-system) für das vollständige Design.

#### Einen Nutzer verwarnen

Sendet eine private DM-Warnung an einen Nutzer bezüglich eines bestimmten Kommentars und protokolliert die Verwarnung atomar im Agentenspeicher. Die Eskalationsrichtlinie der Plattform baut auf diesem Tool auf – zuerst verwarnen, sperren nur bei Wiederholung. Seltener hinter Genehmigungspflicht als `ban_user`, aber erwägen Sie, während der ersten Wochen des Agentenlebens eine Genehmigungspflicht einzurichten. Siehe [Warn user](#tool-warn-user) für die vollständige Seite.

#### Einen Nutzer sperren

Das folgenreichste Tool, das ein Agent aufrufen kann. Sperrt einen Nutzer für eine feste Dauer, optional als Shadow Ban, optional auch die IP sperrend, optional auch alle Kommentare des Nutzers löschend. Die zwei destruktiven Optionen (IP, delete-all) sind im Bearbeitungsformular hinter zusätzlichen Opt-Ins verborgen. In der EU-Region erfordern alle Sperrungen eine menschliche Genehmigung (siehe [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Ziehen Sie in Erwägung, es überall hinter eine Genehmigungspflicht zu stellen. Siehe [Ban user](#tool-ban-user) für die vollständige Seite.

### Unteroptionen des Ban-Tools

Das Ban-Tool bietet zwei destruktive Optionen – delete-all-comments und ban-by-IP – die für das Modell vollständig verborgen sind, bis Sie sie über den Abschnitt **Ban-Optionen** im Bearbeitungsformular aktivieren. Selbst wenn das Modell den Parameter halluziniert, verweigert die Plattform Werte, in die Sie nicht eingewilligt haben. Siehe [Ban user](#tool-ban-user).