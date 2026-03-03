[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Mit Simple SSO können wir dem Kommentar-Widget Informationen über den Benutzer bereitstellen, damit dieser seinen Benutzernamen oder seine E-Mail nicht eingeben muss, um zu kommentieren.

Wir können Simple SSO wie folgt konfigurieren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Der Benutzer wird eingeloggt und im Hintergrund ein SSO-Benutzer erstellt. `createdFromSimpleSSO` wird für den Benutzer auf `true` gesetzt, wenn er von der API abgerufen wurde.

Notes: 

- E-Mail ist der eindeutige Identifikator für Simple SSO.
- Das Angeben einer E-Mail bei Simple SSO ist nicht erforderlich; standardmäßig werden ihre Kommentare jedoch als "Unverified" angezeigt. <b>Wenn keine E-Mail angegeben ist, kann der Benutzer nicht vollständig authentifiziert werden.</b>
- **NEU** Seit Jan 2022: Benutzernamen müssen nicht mehr über ganz fastcomments.com hinweg einzigartig sein
- Simple SSO kann SSO-Benutzer automatisch erstellen und aktualisieren, wenn eine E-Mail angegeben ist und der Benutzer nicht ursprünglich durch Secure SSO erstellt wurde.
- Sie können Abzeichen für den Benutzer mit der Eigenschaft `badgeConfig` angeben. Das Array `badgeIds` enthält die IDs globaler Badges, die dem Benutzer zugeordnet werden sollen. Das Array `pageBadgeIds` enthält Badge-IDs, die auf die aktuelle Seite (`urlId`) beschränkt sind — diese Badges werden nur auf der Seite angezeigt, auf der sie zugewiesen wurden. Wenn `override` auf `true` gesetzt ist, ersetzt es vorhandene angezeigte Badges (globale und seitenbezogene Badges werden dabei unabhängig voneinander überschrieben); ist es `false`, werden die Badges zu den vorhandenen hinzugefügt.

---