Wird ausgelöst, wenn ein Kommentar angepinnt wird.

### Kontext, den der Agent erhält

- Der angepinnte Kommentar.
- Optionaler Thread-/Benutzerverlauf-/Seitenkontext wie konfiguriert.

### Wer löst das aus

- Ein Moderator, der auf der Moderationsseite oder im Kommentar-Widget die Pin-Aktion klickt.
- Ein Agent, der [`pin_comment`](#tools-overview) aufruft.

Schleifenvermeidung: Von Agenten ausgelöste Pin-Ereignisse lösen keine Agententrigger aus. Ein Pin, der von einem Agenten vorgenommen wird, unterbindet jegliche Agentenauslösung für dieses Ereignis, nicht nur die des ursächlichen Agenten.

### Hinweis zum Paar

Anpinnen- und Entpinnen-Ereignisse sind separate Trigger. Abonnieren Sie beide, wenn Sie symmetrisches Verhalten wünschen. Siehe [Trigger: Kommentar entpinnt](#trigger-comment-unpin).