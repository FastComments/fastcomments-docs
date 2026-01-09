[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Mit Simple SSO können wir dem Kommentar-Widget Informationen über den Benutzer bereitstellen, damit dieser beim Kommentieren seinen Benutzernamen oder seine E-Mail nicht eingeben muss.

Wir können Simple SSO wie folgt konfigurieren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Der Benutzer wird angemeldet und im Hintergrund als SSO-Benutzer erstellt. Der Benutzer hat `createdFromSimpleSSO` auf `true` gesetzt, wenn er von der API abgerufen wurde.

Notes: 

- E-Mail ist der eindeutige Bezeichner für Simple SSO.
- Die Angabe einer E-Mail bei Simple SSO ist nicht erforderlich; standardmäßig werden ihre Kommentare jedoch als "Unverified" angezeigt. <b>Wenn keine E-Mail angegeben wird, kann der Benutzer nicht vollständig authentifiziert werden.</b>
- **NEU** Seit Jan. 2022: Benutzernamen müssen nicht mehr für die gesamte fastcomments.com eindeutig sein
- Simple SSO kann SSO-Benutzer automatisch erstellen und aktualisieren, wenn eine E-Mail angegeben ist und der Benutzer nicht ursprünglich über Secure SSO erstellt wurde.
- Sie können Abzeichen für den Benutzer mit der Eigenschaft `badgeConfig` angeben. Das Array `badgeIds` enthält die IDs der Abzeichen, die dem Benutzer zugeordnet werden sollen. Wenn `override` auf `true` gesetzt ist, ersetzt es alle bestehenden Abzeichen, die in Kommentaren angezeigt werden; ist es `false`, werden die Abzeichen zu den vorhandenen hinzugefügt.

---