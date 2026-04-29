**Vorlagen-ID:** `top_comment_pinner`

Der Top-Comment-Pinner überwacht Top-Level-Kommentare, die einen Stimmen-Schwellenwert überschreiten, und pinnt sie – wobei er das zuvor im selben Thread Gepinnte ersetzt.

### Eingebaute initiale Eingabeaufforderung

[inline-code-attrs-start title = 'Initiale Eingabeaufforderung der Top-Comment-Pinner-Vorlage'; type='text' inline-code-attrs-end]
[inline-code-start]
Du pinnst die besten Top-Level-Kommentare in einem Thread. Wenn ein Kommentar den Stimmenschwellenwert erreicht, pinne ihn, sofern der Inhalt substanziell und nicht werblich ist. Entpinne zuerst jeden zuvor gepinnten Kommentar im selben Thread. Pinne keine Antworten, nur Top-Level-Kommentare.
[inline-code-end]

Die Anweisung „pinne keine Antworten“ ist wichtig: Pinnen wirkt auf Threads, daher ist das Pinnen einer Antwort selten sinnvoll. Der Filter „nicht werblich“ verhindert, dass der Agent einen populären Link-Spam-Kommentar pusht.

### Auslöser

- **Ein Kommentar überschreitet einen Stimmen-Schwellenwert** (`COMMENT_VOTE_THRESHOLD`, Standard-Stimmenschwellenwert: 10).

Der Auslöser feuert, wenn die Nettostimmen des Kommentars (`up - down`) den konfigurierten Schwellenwert erreichen. Passe die Zahl im Bearbeitungsformular an, je nachdem, wie aktiv deine Threads sind – 10 ist ein sinnvoller Standard für mäßig aktive Seiten.

### Zulässige Tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinnen ist nicht-destruktiv – es kann sofort rückgängig gemacht werden – daher läuft diese Vorlage normalerweise ohne Genehmigungen.

### Empfohlene Ergänzungen vor dem Livegang

- **Aktiviere "Elternkommentar und vorherige Antworten im selben Thread einbeziehen"** in [Context Options](#context-options). Ohne Thread-Kontext kann der Agent nicht zuverlässig feststellen, ob bereits ein Kommentar gepinnt ist, den er entpinnen müsste.
- **Passe den Stimmen-Schwellenwert** an deine Seite an. Bei stark frequentierten Threads passiert 10 zu oft; bei ruhigen Threads kann 10 vielleicht nie erreicht werden.
- **Ziehe in Betracht, die URL einzuschränken**, wenn du nur in bestimmten Bereichen deiner Seite gepinnte Kommentare möchtest – z. B. in Nachrichtenthreads, aber nicht in Ankündigungs-Threads.

### Hinweis zum doppelten Pinnen

Die Aufforderung im Prompt des Agents weist ihn an, zuerst zu entpinnen, bevor er pinnt, aber wenn das Modell diesen Schritt übersieht, erzwingt die Plattform selbst keine Regel von einem Gepinnten pro Thread (du kannst mehrere haben). Wenn doppeltes Pinnen auf deiner Seite ein Problem darstellt, stelle `pin_comment` hinter eine Genehmigung und prüfe jedes einzelne – oder schreibe eine präzisere Aufforderung.