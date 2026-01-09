Jetzt werden wir deinen benutzerdefinierten FastComments-Code erzeugen. Verwende den untenstehenden Assistenten, um zu konfigurieren, wie FastComments auf deiner GoHighLevel-Seite funktionieren soll:

[snippet id="gohighlevel-wizard"]

### Verschiedene Kommentarbox-Typen

Du kannst die Zeile `TYPE = 'commenting'` konfigurieren, um das verwendete Produkt zu wechseln (zum Beispiel kannst du sie zu `live` für Streaming-Chat oder `collab` für Collab-Chat ändern).

### Die Kommentarbox an der gewünschten Stelle platzieren

Angenommen, du möchtest Kommentarboxen an bestimmten Stellen der Seite platzieren und nicht an den Standardpositionen.
Ändere diese Zeile:

    const TARGET_ELEMENT_ID = ''; // um den Ziel-div-Modus zu verwenden

Zu:

    const TARGET_ELEMENT_ID = 'fc_box'; // um den Ziel-div-Modus zu verwenden

Klicke dann im GHL-Editor auf den "code"-Button und füge dort ein, wo die Kommentare erscheinen sollen:

[inline-code-attrs-start title = 'GoHighLevel FastComments-Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Unterschiedlicher Kommentarbox-Typ pro Seite

Angenommen, du möchtest, dass Benutzer Textstellen hervorheben und diskutieren oder stattdessen die Streaming-Chat-Oberfläche verwenden.

Folge zuerst den oben beschriebenen Schritten in "Die Kommentarbox an der gewünschten Stelle platzieren".

Beachte, dass in diesem kleinen Snippet `type="commenting"` steht.

Wenn du zum Beispiel Collab-Chat aktivieren möchtest, ändere `type` zu `type="collab"`.

### Nur auf bestimmten Seiten anzeigen

Wenn du `TARGET_ELEMENT_ID` nicht setzt, kannst du stattdessen die Variable `VALID_PATTERNS` konfigurieren, um festzulegen, bei welchen URL-Routen die Kommentare angezeigt werden sollen. Standardmäßig werden sie auf Seiten angezeigt, die `/post` in der URL enthalten.

### Collab-Chat konfigurieren

Du kannst dem Collab-Chat anweisen, kollaborative Funktionen nur um HTML innerhalb eines bestimmten Bereichs hinzuzufügen, zum Beispiel, sagen wir, du
fügst den Footer-Code oben ein und fügst dann dieses Div in den Beitrag-/Seiteninhalt ein, um Collab-Chat zu aktivieren:

[inline-code-attrs-start title = 'Collab-Chat mit angegebenem Inhalt'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Dann erhält das Absatz-Element innerhalb des `<div>` Collab-Chat, und sonst nichts auf der Seite. Wenn du keinen Inhalt in das `<div>` einfügst, wird Collab-Chat für den gesamten Beitragstext aktiviert.