Die **Werkzeuge** eines Agenten sind die Aktionen, die er ausführen kann. Das Bearbeitungsformular für den Agenten enthält einen Abschnitt **Erlaubte Werkzeugaufrufe**, in dem Sie die Werkzeuge ankreuzen, die dieser Agent verwenden darf, und einen Abschnitt **Genehmigungen**, in dem Sie die Aktionen ankreuzen, die von einem Menschen genehmigt werden müssen, bevor sie wirksam werden.

Es gibt drei Stufen für jedes Werkzeug:

- **Nicht erlaubt** - der Agent kann es nicht sehen oder verwenden.
- **Erlaubt, keine Genehmigung** - der Agent verwendet es direkt. In der Ausführungshistorie aufgezeichnet.
- **Erlaubt, mit Genehmigung** - der Aufruf des Agenten wird zur menschlichen Prüfung in eine Warteschlange gestellt und läuft nur, wenn ein Mensch ihn genehmigt.

Nicht erlaubte Werkzeuge sind lautlos: der Agent kann nicht danach fragen und die Plattform lehnt sie konsequent ab. Werkzeuge, die der Genehmigung unterliegen, laufen immer über den [Genehmigungs-Posteingang](#approval-workflow).

### Prüfprotokoll für jede Aktion

Jede Aktion, die der Agent ausführt, wird mit einer kurzen Begründung (1–2 Sätze, die erklären, warum) und einer Vertrauensbewertung (0,0–1,0) aufgezeichnet. Beide erscheinen in der [Run Detail View](#run-detail-view) und bei jeder [Genehmigung](#approval-workflow). Das Durchsuchen des Speichers ist die einzige schreibgeschützte Ausnahme: Es wird nicht als Aktion aufgezeichnet und ist immer verfügbar, unabhängig von der Allowlist.

### Werkzeugreferenz

#### Kommentare posten

Ermöglicht dem Agenten, einen Kommentar als er selbst zu posten. Der Kommentar wird öffentlich unter dem Anzeigenamen des Agenten angezeigt. Wird von Begrüßungs- und Zusammenfassungsagenten verwendet. Umkehrbar – jeder Moderator kann einen schlechten Kommentar entfernen. Hinterlegen Sie es hinter einer Genehmigung, wenn Ihre Community möchte, dass jede öffentlich sichtbare Nachricht von einem Menschen geprüft wird.

#### Einen Kommentar bearbeiten

Ermöglicht dem Agenten, den Text eines im Geltungsbereich liegenden Kommentars umzuschreiben. Der Originaltext wird im Prüfprotokoll des Kommentars aufbewahrt. Nur für enge Anwendungsfälle reservieren – das Schwärzen von vom Nutzer geleakten personenbezogenen Daten (PII) oder das Ändern einer eigenen vorherigen Antwort des Agenten. Nicht zum Umschreiben von Meinungen oder zur Abschwächung des Tons. Siehe [Edit comment](#tool-edit-comment) für die komplette Seite.

#### Auf Kommentare abstimmen

Ermöglicht dem Agenten, einen Kommentar hoch- oder runterzustimmen. Die Stimme zählt zur Gesamtzahl der Stimmen des Kommentars wie jede andere Stimme. Die meisten Communities bevorzugen es, keine Bots abstimmen zu lassen; in keiner Starter-Vorlage aktiviert. Wenn Sie es zulassen, ist Abstimmen umkehrbar.

#### Einen Kommentar anpinnen / abpinnen

Ermöglicht dem Agenten, einen Kommentar oben auf der Seite anzuheften oder einen bereits angehefteten Kommentar zu lösen. Die Plattform erzwingt keine Regel von einem Pin pro Thread, daher sollte ein anpinnender Agent angewiesen werden, zuerst den vorherigen angepinnten Kommentar zu lösen. Um herauszufinden, was bereits auf derselben Seite angepinnt ist, kann der Agent das schreibgeschützte Tool `get_pinned_comments` aufrufen (siehe unten). Wird von der Top Comment Pinner template verwendet.

#### Einen Kommentar sperren / entsperren

Ermöglicht dem Agenten, weitere Antworten unter einem Kommentar zu verhindern oder Antworten wiederherzustellen. Der gesperrte Kommentar bleibt sichtbar. Nützlich für Abkühlphasen bei aufgeheizten Threads, in Kombination mit einer verzögerten Entsperrung. Um herauszufinden, was derzeit auf derselben Seite gesperrt ist, kann der Agent das schreibgeschützte Tool `get_locked_comments` aufrufen (siehe unten).

#### Als Spam markieren / Spam-Markierung entfernen

Ermöglicht dem Agenten, einen Kommentar als Spam zu markieren (und damit für Leser zu verbergen und den Spam-Klassifikator zu füttern) oder dieses Flag zu entfernen. Das Standardwerkzeug für jeden Moderationsagenten. Umkehrbar.

#### Kommentar genehmigen / Freigabe aufheben

Ermöglicht dem Agenten, einen gehaltenen Kommentar für Leser sichtbar zu machen oder einen bereits sichtbaren Kommentar zu verbergen. Am nützlichsten auf Tenants, die neue Kommentare zur Moderatorenprüfung zurückhalten.

#### Einen Kommentar als geprüft markieren

Ein Warteschlangen-Status-Werkzeug: markiert einen Kommentar als „ein Moderator (oder Agent) hat das hier angesehen“. Ändert die Sichtbarkeit nicht. Geringes Risiko; selten hinter einer Genehmigung.

#### Ein Abzeichen vergeben

Ermöglicht dem Agenten, einem Benutzer ein für Ihren Tenant konfiguriertes Abzeichen zu geben. Durch einen Moderator rückgängig machbar. Wenn dieses Werkzeug aktiviert ist, kann der Agent die Abzeichen Ihres Tenants sehen und das passende selbst auswählen, sodass Sie keine Abzeichen-IDs in Ihre Community-Richtlinien oder den initialen Prompt einfügen müssen. Um zu steuern, welches Abzeichen für welches Verhalten vergeben wird, beziehen Sie sich im Prompt auf die Abzeichen anhand ihres **Anzeigenbezeichnung**.

#### E-Mail senden

Ermöglicht dem Agenten, eine Nur-Text-E-Mail an den Autor eines Kommentars im Geltungsbereich des Triggers zu senden. Der Agent sieht niemals die E-Mail-Adresse des Empfängers – er wählt einen Kommentar aus und die Plattform liefert an die Adresse, die dieser Kommentator beim Posten hinterlassen hat. Die Absenderadresse ist die Marken-Absenderadresse Ihres Tenants (mit DKIM), wenn die Domain des Kommentars mit einer konfigurierten Domain übereinstimmt, ansonsten die Plattform-Standardeinstellung. Sparsam einsetzen – E-Mail ist das Werkzeug mit der höchsten Hürde und schlechte E-Mails sind schwer rückgängig zu machen.

#### Speichern / Durchsuchen des Agenten-Speichers

Zwei gekoppelte Werkzeuge, die einen gemeinsamen Notizpool über den Benutzer lesen und schreiben, für den ein Trigger ausgelöst wurde. Der Speicher wird zwischen allen Agenten in Ihrem Tenant geteilt, sodass die Notizen eines Triage-Agenten die Entscheidungen eines Moderator-Agenten informieren. Durchsuchen ist schreibgeschützt und immer verfügbar; Speichern wird selten hinter einer Genehmigung verborgen. Siehe [Agenten-Speichersystem](#agent-memory-system) für das vollständige Design.

#### Gepinnte Kommentare abrufen / Gesperrte Kommentare abrufen

Zwei schreibgeschützte Entdeckungswerkzeuge, die die angepinnten (oder gesperrten) Kommentare auf derselben Seite (`urlId`) auflisten, auf der der Trigger ausgelöst wurde. Sie nehmen keine Argumente entgegen – die Seite wird aus dem Trigger-Kontext gelesen, sodass der Agent nicht auf andere Seiten pivotieren kann. Verwenden Sie sie, wenn ein Agent auf einen Kommentar reagieren muss, der bereits angepinnt oder gesperrt ist – typischerweise der erste Aufruf vor `unpin_comment` oder `unlock_comment`, oder bevor ein neuer Kommentar angepinnt wird, damit der bestehende zuerst gelöst werden kann.

Jedes Werkzeug wird separat in **Erlaubte Werkzeugaufrufe** gesteuert (der Administrator aktiviert `List pinned comments on the current page` oder `List locked comments on the current page`). Sie können nicht hinter einer Genehmigung versteckt werden – schreibgeschützte Werkzeuge haben keine Nebenwirkung, die genehmigt werden müsste. Das Aufrufen dieser Werkzeuge wird nicht als Aktion in der Ausführungshistorie aufgezeichnet; nur der daraus resultierende `unpin_comment` / `unlock_comment` / `pin_comment` Aufruf (falls vorhanden) erscheint. Die Liste ist auf die letzten 20 Treffer pro Aufruf begrenzt.

Wichtig zu verstehen: wenn eines dieser Werkzeuge eine commentId zurückgibt, wird diese commentId dem pro-Ausführung-Bereich des Agenten hinzugefügt, sodass der Folgeaufruf `unpin_comment` / `unlock_comment` gegen die Sicherheitsprüfung der Plattform für Werkzeugziele validiert wird. Ohne zuvor das Entdeckungswerkzeug aufzurufen, kann der Agent nicht auf Kommentare außerhalb des unmittelbaren Triggerscope reagieren. Daher erhält ein Agent im Stil "Abpinnen" typischerweise beide Werkzeuge aktiviert (z. B. `get_pinned_comments` plus `unpin_comment`).

#### Einen Nutzer warnen

Sendet eine private Direktnachricht (DM) an einen Nutzer über einen bestimmten Kommentar und zeichnet atomar die Warnung im Agenten-Speicher auf. Die Eskalationspolitik der Plattform ist um dieses Werkzeug herum aufgebaut – zuerst warnen, nur bei Wiederholung sperren. Siehe [Warn user](#tool-warn-user) für die komplette Seite.

#### Einen Nutzer sperren

Das folgenreichste Werkzeug, das ein Agent aufrufen kann. Sperrt einen Nutzer für eine feste Dauer, optional als Shadowban, optional auch die IP sperrend, optional auch alle Kommentare des Nutzers löschend. Die beiden destruktiven Optionen (IP, alle löschen) sind auf dem Bearbeitungsformular hinter zusätzlichen Opt-ins verborgen. In der Region EU erfordern alle Sperren eine menschliche Genehmigung (siehe [EU DSA Artikel 17-Konformität](#eu-dsa-compliance)). Siehe [Ban user](#tool-ban-user) für die komplette Seite.

### Unteroptionen des Sperr-Tools

Das Sperr-Werkzeug bietet zwei destruktive Optionen – delete-all-comments und ban-by-IP – die dem Modell vollständig verborgen bleiben, bis Sie sie über den Abschnitt **Sperroptionen** im Bearbeitungsformular aktivieren. Selbst wenn das Modell den Parameter halluziniert, verweigert die Plattform Werte, in die Sie nicht eingewilligt haben. Siehe [Ban user](#tool-ban-user).