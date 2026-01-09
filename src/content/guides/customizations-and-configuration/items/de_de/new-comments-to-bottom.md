[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Standardmäßig erscheinen neue Live-Kommentare oben in der Kommentarliste, sobald sie in Echtzeit veröffentlicht werden.

Wenn diese Option aktiviert ist, werden neue Live-Kommentare stattdessen am unteren Ende der Liste hinzugefügt. Dies beeinflusst, wie Kommentare erscheinen, wenn sie live gepostet werden, während Benutzer den Kommentarthread ansehen.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Wenn diese Einstellung aktiviert ist:
- Von anderen Benutzern gepostete neue Live-Kommentare werden am unteren Ende der Kommentarliste erscheinen
- Benutzer sehen in Echtzeit, wie neue Kommentare unter bestehenden Kommentaren erscheinen
- Dies betrifft nur Live-Kommentar-Aktualisierungen - nicht das initiale Laden der Seite
- Dies kann helfen, den Lesefluss zu erhalten, wenn Benutzer einer Diskussion folgen

Beachten Sie, dass diese Einstellung nur beeinflusst, wo neue Live-Kommentare platziert werden, wenn sie in Echtzeit eintreffen. Sie beeinflusst nicht die anfängliche Sortierreihenfolge beim Laden der Seite.