Ein Agenten **Tools** sind die Aktionen, die er ausführen kann. Das Bearbeitungsformular des Agents hat einen **Abschnitt „Erlaubte Tool-Aufrufe“**, in dem Sie die Tools ankreuzen, die dieser Agent verwenden darf, und einen **Abschnitt „Genehmigungen“**, in dem Sie die Aktionen ankreuzen, die von einem Menschen genehmigt werden müssen, bevor sie wirksam werden.

Es gibt drei Stufen für jedes Tool:

- **Nicht erlaubt** - der Agent kann es weder sehen noch verwenden.
- **Erlaubt, keine Genehmigung erforderlich** - der Agent verwendet es direkt. In der Ausführungshistorie protokolliert.
- **Erlaubt, mit Genehmigung erforderlich** - der Aufruf des Agents wird zur menschlichen Überprüfung in die Warteschlange gestellt und läuft nur, wenn ein Mensch zustimmt.

Nicht erlaubte Tools sind stumm: der Agent kann nicht danach fragen und die Plattform lehnt sie strikt ab. Genehmigungsgegate Tools laufen immer über den [Genehmigungs-Posteingang](#approval-workflow).

### Audit-Trail für jede Aktion

Jede Aktion des Agents wird mit einer kurzen Begründung (1–2 Sätze, die erklären, warum) und einer Vertrauensbewertung (0.0–1.0) protokolliert. Beides erscheint in der [Ansicht der Ausführungsdetails](#run-detail-view) und bei jeder [Genehmigung](#approval-workflow). Das Durchsuchen des Speichers ist die einzige read-only-Ausnahme: es wird nicht als Aktion protokolliert und ist unabhängig von der Zulassungsliste immer verfügbar.

### Tool-Referenz

#### Kommentare posten

Ermöglicht dem Agenten, einen Kommentar in seinem Namen zu posten. Der Kommentar wird öffentlich unter dem Anzeigenamen des Agents angezeigt. Wird von Begrüßungs- und Zusammenfassungsagenten verwendet. Umkehrbar – jeder Moderator kann einen schlechten Kommentar entfernen. Normalerweise ohne Genehmigung erlaubt; schränken Sie es ein, wenn in Ihrer Community jede öffentlich sichtbare Nachricht von einem Menschen überprüft werden muss.

#### Kommentar bearbeiten

Ermöglicht dem Agenten, den Text eines relevanten Kommentars umzuschreiben. Der Originaltext wird im Audit-Log des Kommentars erhalten. Nur für enge Fälle reservieren – das Schwärzen von vom Nutzer geleakten personenbezogenen Daten (PII) oder das Ändern der eigenen früheren Antwort des Agents. Nicht zum Umschreiben von Meinungen oder Abschwächen des Tons. **Erwägen Sie unbedingt, dies hinter einer Genehmigung zu platzieren.** Siehe [Kommentar bearbeiten](#tool-edit-comment) für die vollständige Seite.

#### Kommentare bewerten

Ermöglicht dem Agenten, für einen Kommentar positiv oder negativ zu stimmen. Die Stimme zählt wie jede andere Stimme zur Gesamtwertung des Kommentars. Die meisten Communities bevorzugen, dass Bots nicht abstimmen; in keiner Starter-Vorlage aktiviert. Wenn Sie es erlauben, ist die Abstimmung rückgängig zu machen.

#### Anpinnen / Abpinnen eines Kommentars

Ermöglicht dem Agenten, einen Kommentar oben auf der Seite anzupinnen oder einen bereits angepinnten Kommentar zu lösen. Die Plattform erzwingt keine Ein-Pin-pro-Thread-Regel, daher sollte ein anpinnender Agent angewiesen werden, zuerst den zuvor angepinnten Kommentar zu lösen. Wird vom Top Comment Pinner template verwendet. Rückgängig machbar; normalerweise ohne Genehmigung erlaubt.

#### Kommentar sperren / entsperren

Ermöglicht dem Agenten, weitere Antworten unter einem Kommentar zu verhindern oder Antworten wiederherzustellen. Der gesperrte Kommentar bleibt sichtbar. Nützlich zur Abkühlung heißer Threads, kombiniert mit einer verzögerten Entsperrung. Rückgängig machbar, aber für Ihre Community sichtbar; erwägen Sie eine Genehmigung in hochriskanten Communities.

#### Als Spam markieren / Spam-Markierung entfernen

Ermöglicht dem Agenten, einen Kommentar als Spam zu markieren (ihn vor Lesern zu verbergen und den Spam-Klassifizierer zu füttern) oder diese Markierung zu entfernen. Das Standardwerkzeug für jeden Moderations-Agenten. Rückgängig machbar. Erwägen Sie dringend, dies in den ersten Wochen hinter einer Genehmigung zu platzieren, während Sie Vertrauen in den Agenten aufbauen.

#### Kommentar freigeben / Freigabe zurückziehen

Ermöglicht dem Agenten, einen zurückgehaltenen Kommentar den Lesern zu zeigen oder einen bereits sichtbaren Kommentar zu verbergen. Am nützlichsten bei Mandanten, die neue Kommentare zur Moderatorprüfung zurückhalten. Hohes Risiko beim Zurückziehen der Freigabe eines sichtbaren Kommentars – erwägen Sie eine Genehmigung.

#### Kommentar als geprüft markieren

Ein Queue-State-Tool: markiert einen Kommentar als „ein Moderator (oder Agent) hat dies angesehen“. Ändert die Sichtbarkeit nicht. Geringes Risiko; selten genehmigungspflichtig.

#### Ein Abzeichen vergeben

Ermöglicht dem Agenten, einem Nutzer ein Abzeichen aus der Abzeichenkonfiguration Ihres Mandanten zu vergeben. Vom Moderator rückgängig machbar. Selten genehmigungspflichtig. Der Agent muss die Abzeichen-ID kennen, fügen Sie also die relevanten IDs in Ihre [Community-Richtlinien](#community-guidelines) oder [Initiale Eingabeaufforderung](#personality-prompt) ein.

#### E-Mail senden

Ermöglicht dem Agenten, eine Plain-Text-E-Mail von `noreply@fastcomments.com` an eine von ihm gewählte Adresse zu senden. Sparsam verwenden – E-Mails sind eine hoch riskante Maßnahme und schlechte E-Mails sind schwer rückgängig zu machen. Erwägen Sie dringend, dies hinter eine Genehmigung zu legen, und leiten Sie Genehmigungs-E-Mails an die Person weiter, der das Postfach gehört, an das der Agent letztlich schreiben wird.

#### Agenten-Speicher speichern / durchsuchen

Zwei gekoppelte Werkzeuge, die einen gemeinsamen Notizen-Pool über den Benutzer lesen und schreiben, für den ein Trigger ausgelöst wurde. Der Speicher wird von allen Agents in Ihrem Mandanten geteilt, sodass die Notizen eines Triage-Agenten die Entscheidungen eines Moderatoren-Agenten beeinflussen. Die Suche ist schreibgeschützt und immer verfügbar; Speichern ist selten genehmigungspflichtig. Siehe [Agenten-Speichersystem](#agent-memory-system) für das vollständige Design.

#### Einen Nutzer warnen

Sendet eine private Direktnachricht mit einer Warnung an einen Nutzer bezüglich eines bestimmten Kommentars und protokolliert die Warnung atomar im Agentenspeicher. Die Eskalationspolitik der Plattform ist um dieses Werkzeug herum aufgebaut – zunächst warnen, nur bei Wiederholung sperren. Seltener genehmigungspflichtig als `ban_user`, aber erwägen Sie eine Genehmigung in den ersten Wochen der Laufzeit eines Agents. Siehe [Nutzer warnen](#tool-warn-user) für die vollständige Seite.

#### Einen Nutzer bannen

Das folgenreichste Werkzeug, das ein Agent aufrufen kann. Sperrt einen Nutzer für eine feste Dauer, optional als Shadow-Ban, optional auch Sperrung der IP, optional auch das Löschen aller Kommentare des Nutzers. Die zwei destruktiven Optionen (IP, Löschen aller Kommentare) sind hinter zusätzlichen Opt-ins im Bearbeitungsformular verborgen. In der EU-Region erfordern alle Sperren eine menschliche Genehmigung (siehe [EU-DSA Artikel 17 Konformität](#eu-dsa-compliance)). Erwägen Sie dringend, dies überall hinter eine Genehmigung zu stellen. Siehe [Nutzer sperren](#tool-ban-user) für die vollständige Seite.

### Unteroptionen des Ban-Werkzeugs

Das Ban-Werkzeug bietet zwei destruktive Optionen an - delete-all-comments und ban-by-IP - die für das Modell vollständig verborgen sind, bis Sie sie über den **Ban-Optionen**-Abschnitt im Bearbeitungsformular aktivieren. Selbst wenn das Modell den Parameter halluziniert, lehnt die Plattform Werte ab, in die Sie nicht eingewilligt haben. Siehe [Nutzer sperren](#tool-ban-user).

---