Die **Tools** eines Agenten sind die Aktionen, die er ausführen kann. Das Bearbeitungsformular für Agenten enthält einen Abschnitt **Zugelassene Tool-Aufrufe**, in dem Sie die Tools ankreuzen, die dieser Agent verwenden darf, und einen Abschnitt **Genehmigungen**, in dem Sie die Aktionen ankreuzen, die von einem Menschen genehmigt werden müssen, bevor sie wirksam werden.

Es gibt drei Stufen für jedes Tool:

- **Nicht erlaubt** - der Agent kann es nicht sehen oder verwenden.
- **Erlaubt, keine Genehmigung** - der Agent verwendet es direkt. Wird in der Ausführungshistorie protokolliert.
- **Erlaubt, mit Genehmigung** - der Aufruf des Agenten wird zur menschlichen Überprüfung in eine Warteschlange gestellt und läuft erst, wenn ein Mensch ihn genehmigt.

Nicht erlaubte Tools sind stumm: der Agent kann sie nicht anfordern und die Plattform lehnt sie unmittelbar ab. Tools, die eine Genehmigung erfordern, laufen immer über den [Genehmigungs-Posteingang](#approval-workflow).

### Prüfprotokoll bei jeder Aktion

Jede Aktion, die der Agent ausführt, wird mit einer kurzen Begründung (1–2 Sätze, die erklären, warum) und einer Vertrauensbewertung (0,0–1,0) aufgezeichnet. Beides erscheint in der [Ansicht der Ausführungsdetails](#run-detail-view) und bei jeder [Genehmigung](#approval-workflow). Die Suche im Speicher ist die einzige schreibgeschützte Ausnahme: Sie wird nicht als Aktion aufgezeichnet und ist unabhängig von der Allowlist immer verfügbar.

### Tool-Referenz

#### Kommentare posten

Ermöglicht dem Agenten, im eigenen Namen einen Kommentar zu posten. Der Kommentar wird öffentlich unter dem Anzeigenamen des Agenten angezeigt. Wird von Begrüßungs- und Zusammenfassungsagenten verwendet. Rückgängig machbar – jeder Moderator kann einen schlechten Kommentar entfernen. Normalerweise ohne Genehmigung erlaubt; sperren Sie es, wenn Ihre Community jede öffentlich sichtbare Nachricht von Menschen überprüfen lassen muss.

#### Einen Kommentar bearbeiten

Ermöglicht dem Agenten, den Text eines im Geltungsbereich liegenden Kommentars umzuschreiben. Der Originaltext bleibt im Prüfprotokoll des Kommentars erhalten. Nur für enge Fälle reservieren – das Schwärzen von vom Nutzer geleakten PII oder das Korrigieren einer eigenen vorherigen Agenten-Antwort. Nicht zum Umschreiben von Meinungen oder zur Abschwächung des Tons. **Ziehen Sie stark in Betracht, dies hinter einer Genehmigung zu verbergen.** Siehe [Kommentar bearbeiten](#tool-edit-comment) für die vollständige Seite.

#### Kommentare bewerten

Ermöglicht dem Agenten, für einen Kommentar positiv oder negativ zu stimmen. Die Stimme zählt wie jede andere Stimme zur Abstimmungszahl des Kommentars. Die meisten Communities bevorzugen es, dass Bots nicht abstimmen; ist in keiner Starter-Vorlage aktiviert. Wenn Sie es zulassen, ist das Abstimmen rückgängig machbar.

#### Kommentar anpinnen / lösen

Ermöglicht dem Agenten, einen Kommentar oben auf der Seite anzupinnen oder einen bereits angepinnten Kommentar zu lösen. Die Plattform erzwingt keine Regel von einem Pin pro Thread, daher sollte ein anpinnder Agent angewiesen werden, zuerst den vorherigen angepinnten Kommentar zu lösen. Verwendet von der Top Comment Pinner template. Rückgängig machbar; normalerweise ohne Genehmigung erlaubt.

#### Kommentar sperren / entsperren

Ermöglicht dem Agenten, weitere Antworten unter einem Kommentar zu verhindern oder Antworten wieder zuzulassen. Der gesperrte Kommentar bleibt sichtbar. Nützlich für Abkühlphasen in hitzigen Threads, in Kombination mit einer späteren Entsperrung. Rückgängig machbar, aber für Ihre Community sichtbar; ziehen Sie in hochriskanten Communities eine Genehmigung in Betracht.

#### Als Spam markieren / Spam-Markierung entfernen

Ermöglicht dem Agenten, einen Kommentar als Spam zu markieren (und ihn vor Lesern zu verbergen sowie den Spam-Klassifikator zu füttern) oder dieses Flag zu entfernen. Das Brot-und-Butter-Tool für jeden Moderationsagenten. Rückgängig machbar. Ziehen Sie in den ersten Wochen, während Sie Vertrauen in den Agenten aufbauen, stark in Betracht, dies hinter einer Genehmigung zu verbergen.

#### Kommentar freigeben / Freigabe aufheben

Ermöglicht dem Agenten, einen zurückgehaltenen Kommentar den Lesern zu zeigen oder einen bereits sichtbaren Kommentar zu verbergen. Am nützlichsten bei Mandanten, die neue Kommentare zur Moderatorenüberprüfung zurückhalten. Hohe Risiken beim Aufheben der Freigabe eines sichtbaren Kommentars – ziehen Sie überall eine Genehmigung in Betracht.

#### Einen Kommentar als geprüft markieren

Ein Queue-Status-Tool: markiert einen Kommentar als „ein Moderator (oder Agent) hat sich das angesehen“. Ändert die Sichtbarkeit nicht. Geringes Risiko; selten hinter einer Genehmigung verborgen.

#### Auszeichnen mit einem Abzeichen

Ermöglicht dem Agenten, einem Nutzer ein für Ihren Mandanten konfiguriertes Abzeichen zu vergeben. Rückgängig machbar durch einen Moderator. Selten hinter einer Genehmigung verborgen. Wenn dieses Tool aktiviert ist, kann der Agent die Abzeichen Ihres Mandanten sehen und selbst das richtige auswählen, sodass Sie keine Abzeichenkennungen in Ihre Community-Richtlinien oder das Anfangs-Prompt einfügen müssen. Wenn Sie steuern möchten, welches Abzeichen für welches Verhalten vergeben wird, beziehen Sie sich im Prompt auf die Abzeichen über ihre **Anzeigenbezeichnung**.

#### E-Mail senden

Ermöglicht dem Agenten, eine Klartext-E-Mail an den Autor eines Kommentars im Auslösebereich zu senden. Der Agent sieht niemals die E-Mail-Adresse des Empfängers – er wählt einen Kommentar aus und die Plattform liefert an die Adresse, die der Kommentator beim Posten angegeben hat. Die Absenderadresse ist der gebrandete Absender Ihres Mandanten (mit DKIM), wenn die Domain des Kommentars einer konfigurierten Domain entspricht, andernfalls die Plattform-Standardeinstellung. Sparsam einsetzen – E-Mail ist das Tool mit der höchsten Reibung und schlechte E-Mails sind schwer rückgängig zu machen. Ziehen Sie stark in Betracht, dies hinter einer Genehmigung zu verbergen, und leiten Sie Genehmigungs-E-Mails an die Person weiter, die den Posteingang besitzt, an den der Agent am Ende senden wird.

#### Agentenspeicher speichern / durchsuchen

Zwei gekoppelte Tools, die einen gemeinsamen Notizpool über den Nutzer lesen und schreiben, für den ein Auslöser lief. Der Speicher wird von allen Agenten in Ihrem Mandanten geteilt, sodass die Notizen eines Triage-Agenten die Entscheidungen eines Moderator-Agenten informieren. Die Suche ist nur lesend und immer verfügbar; das Speichern ist selten hinter einer Genehmigung verborgen. Siehe [Agentenspeichersystem](#agent-memory-system) für das vollständige Design.

#### Einen Nutzer verwarnen

Sendet eine private Direktnachricht-Warnung an einen Nutzer wegen eines bestimmten Kommentars und zeichnet die Verwarnung atomar im Agentenspeicher auf. Die Eskalationspolitik der Plattform ist um dieses Tool herum aufgebaut – zuerst verwarnen, sperren nur bei Wiederholung. Seltener hinter einer Genehmigung verborgen als `ban_user`, aber ziehen Sie in den ersten Wochen des Lebens eines Agenten eine Genehmigung in Betracht. Siehe [Nutzer verwarnen](#tool-warn-user) für die vollständige Seite.

#### Einen Nutzer sperren

Das folgenreichste Tool, das ein Agent aufrufen kann. Sperrt einen Nutzer mit fester Dauer, optional als Shadow Ban, optional auch Sperrung der IP, optional auch Löschen aller Kommentare des Nutzers. Die beiden destruktiven Optionen (IP, alle löschen) sind im Bearbeitungsformular hinter zusätzlichen Opt-ins verborgen. Selbst wenn das Modell den Parameter halluziniert, lehnt die Plattform Werte ab, die Sie nicht zugelassen haben. In der EU-Region erfordern alle Sperren eine menschliche Genehmigung (siehe [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Ziehen Sie überall stark in Betracht, dies hinter einer Genehmigung zu verbergen. Siehe [Nutzer sperren](#tool-ban-user) für die vollständige Seite.

### Unteroptionen des Sperr-Tools

Das Ban-Tool bietet zwei destruktive Optionen – delete-all-comments und ban-by-IP – die dem Modell vollständig verborgen sind, bis Sie sie über die **Ban options**-Sektion im Bearbeitungsformular aktivieren. Selbst wenn das Modell den Parameter halluziniert, lehnt die Plattform Werte ab, die Sie nicht aktiviert haben. Siehe [Nutzer sperren](#tool-ban-user).