**Template ID:** `top_comment_pinner`

Der Top Comment Pinner beobachtet oberste Kommentare, die eine Stimmen-Schwelle überschreiten, und pinnt sie – wodurch das zuvor im selben Thread angepinnte Element ersetzt wird.

Die eingebaute Prompt weist den Agenten an, Antworten zu überspringen (Anpinnen funktioniert auf Threads, daher ist das Anpinnen einer Antwort selten nützlich) und werbliche Inhalte herauszufiltern (damit der Agent beliebten Link-Spam nicht fördert).

### Triggers

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

Der Trigger wird ausgelöst, wenn die Nettostimmen des Kommentars (`up - down`) die konfigurierte Schwelle erreichen. Passen Sie die Zahl im Bearbeitungsformular an, je nachdem wie aktiv Ihre Threads sind – 10 ist ein sinnvoller Standardwert für mäßig aktive Websites.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinning ist nicht-destruktiv – es kann sofort rückgängig gemacht werden – daher läuft diese Vorlage in der Regel ohne Genehmigungen.

### Recommended additions before going live

- **Tick "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). Ohne Thread-Kontext kann der Agent nicht zuverlässig erkennen, ob bereits ein angepinnter Kommentar vorhanden ist, den es zu lösen gilt.
- **Adjust the vote threshold** to your site. On busy threads 10 happens too often; on quiet threads 10 may never happen.
- **Consider scoping by URL** if you only want pinned comments on certain sections of your site - news threads, say, but not announcement threads.

### Note on duplicate pinning

The agent's prompt instructs it to unpin first before pinning, but if the model misses that step the platform itself does not enforce a one-pinned-per-thread rule (you can have multiple). If duplicate pinning is a problem on your site, gate `pin_comment` behind approval and review each one - or write a tighter prompt.

---