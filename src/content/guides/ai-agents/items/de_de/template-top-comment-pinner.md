**Template-ID:** `top_comment_pinner`

Der Top Comment Pinner überwacht Kommentare auf oberster Ebene, die eine Stimmschwelle überschreiten, und pinnt sie – wobei er ersetzt, was zuvor im selben Thread gepinnt war.

Der integrierte Prompt weist den Agenten an, Antworten zu überspringen (Pinnen funktioniert auf Thread-Ebene, daher ist das Pinnen einer Antwort selten sinnvoll) und Werbeinhalte herauszufiltern (damit der Agent beliebten Link-Spam nicht verstärkt).

### Auslöser

- **Ein Kommentar überschreitet eine Stimmschwelle** (`COMMENT_VOTE_THRESHOLD`, Standard-Stimmschwelle: 10).

Der Auslöser wird aktiviert, wenn die Netto-Stimmen des Kommentars (`up - down`) die konfigurierte Schwelle erreichen. Passen Sie die Zahl im Bearbeitungsformular an die Aktivität Ihrer Threads an – 10 ist ein sinnvoller Standardwert für mäßig aktive Websites.

### Erlaubte Tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinnen ist nicht destruktiv – es kann sofort rückgängig gemacht werden – daher läuft diese Vorlage normalerweise ohne Freigaben.

### Empfohlene Ergänzungen vor dem Livegang

- **Aktivieren Sie „Übergeordneten Kommentar und vorherige Antworten im selben Thread einbeziehen“** in den [Context Options](#context-options). Ohne Thread-Kontext kann der Agent nicht zuverlässig erkennen, ob bereits ein gepinnter Kommentar zum Lösen vorhanden ist.
- **Passen Sie die Stimmschwelle** an Ihre Website an. In stark frequentierten Threads tritt 10 zu oft auf; in ruhigen Threads tritt 10 möglicherweise nie auf.
- **Erwägen Sie eine Eingrenzung nach URL**, wenn Sie gepinnte Kommentare nur in bestimmten Bereichen Ihrer Website wünschen – etwa in News-Threads, aber nicht in Ankündigungs-Threads.

### Hinweis zum doppelten Pinnen

Der Prompt des Agenten weist ihn an, zuerst zu entpinnen, bevor er pinnt, aber wenn das Modell diesen Schritt übersieht, erzwingt die Plattform selbst keine Regel von einem gepinnten Kommentar pro Thread (mehrere sind möglich). Wenn doppeltes Pinnen auf Ihrer Website ein Problem darstellt, schalten Sie `pin_comment` hinter eine Freigabe und prüfen Sie jeden einzeln – oder schreiben Sie einen strengeren Prompt.