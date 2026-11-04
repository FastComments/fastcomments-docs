[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Mit Simple SSO können wir dem Kommentar‑Widget Informationen über den Benutzer bereitstellen, sodass dieser keinen Benutzernamen oder keine E‑Mail‑Adresse eingeben muss, um zu kommentieren.

Wir können Simple SSO wie folgt konfigurieren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Der Benutzer wird eingeloggt sein und im Hintergrund einen SSO‑Benutzer erstellen. Der Benutzer hat `createdFromSimpleSSO` auf `true` gesetzt, wenn er über die API abgerufen wird.

Hinweise: 

- E‑Mail ist der eindeutige Bezeichner für Simple SSO.
- Das Bereitstellen einer E‑Mail mit Simple SSO ist nicht erforderlich, jedoch werden ihre Kommentare standardmäßig als "Unverified" angezeigt. <b>Wenn keine E‑Mail angegeben wird, kann der Benutzer nicht vollständig authentifiziert werden.</b>
- **NEU** Seit Jan 2022: Benutzernamen müssen nicht mehr über die gesamte fastcomments.com hinweg eindeutig sein
- Simple SSO kann SSO‑Benutzer automatisch erstellen und aktualisieren, wenn eine E‑Mail angegeben wird und der Benutzer nicht ursprünglich von Secure SSO erstellt wurde. Wenn ein Simple‑SSO‑Payload einen von Secure SSO oder der API erstellten Benutzer benennt, bleibt dieser Benutzer unverändert und der Besucher erhält einen separaten Benutzer.
- Sie können dem Benutzer Badges über die Eigenschaft `badgeConfig` zuweisen. Das Array `badgeIds` enthält die IDs globaler Badges, die dem Benutzer zugeordnet werden sollen. Das Array `pageBadgeIds` enthält Badge‑IDs, die auf die aktuelle Seite (`urlId`) beschränkt sind – diese Badges werden nur auf der Seite angezeigt, der sie zugewiesen wurden. Wenn `override` auf `true` gesetzt ist, ersetzt es vorhandene angezeigte Badges (globale und seitenbezogene Badges werden unabhängig voneinander überschrieben); wenn `false`, werden sie zu den bestehenden Badges hinzugefügt.

#### Simple SSO Einstellungen

Simple SSO ist jederzeit verfügbar. Zwei Einstellungen steuern, wie sehr FastComments ihr vertraut. Sie können sie finden, indem Sie in einer <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> nach SSO suchen. Beide sind standardmäßig deaktiviert und gelten unabhängig davon, ob "Require SSO To View Comments" aktiviert ist.

Konten, die bereits Simple SSO nutzten, als diese Einstellungen hinzugefügt wurden, haben beide aktiviert, sodass sich für sie nichts geändert hat.

Nur die Anpassungsregel, die am besten zu einer Seite passt, wird angewendet. Wenn Sie also mehr als eine Regel haben, setzen Sie diese in jeder von ihnen. Eine neue Regel beginnt mit aktivierten Einstellungen, wenn eine Ihrer anderen Regeln sie aktiviert hat.

##### Auto Verify Comments via Simple SSO

Wenn diese Option aktiviert ist, werden Kommentare von Simple‑SSO‑Benutzern sofort verifiziert.

Wenn diese Option deaktiviert ist, werden Kommentare von Simple‑SSO‑Benutzern als "Unverified" angezeigt, bis der Benutzer einen Link anklickt, den wir an die im Simple‑SSO‑Payload angegebene Adresse senden. Wenn Sie nur verifizierte Kommentare automatisch freigeben, warten sie ebenfalls auf diesen Link. Gibt es keine E‑Mail im Payload, bleibt der Kommentar unverifiziert.

Diese Einstellung betrifft nur Kommentare. Stimmen von Simple‑SSO‑Benutzern werden immer sofort gezählt.

##### Allow Simple SSO Admins

Wenn diese Option aktiviert ist, kann ein Simple‑SSO‑Benutzer Administrator oder Moderator Ihres Kontos sein:

- Ein Simple‑SSO‑Benutzer, der Administrator‑ oder Moderator‑Berechtigungen hat, behält sie.
- Ein Simple‑SSO‑Benutzer mit derselben E‑Mail wie einer Ihrer Administratoren oder Moderatoren übernimmt deren Berechtigungen.

Wenn diese Option deaktiviert ist, gewährt Simple SSO diese Berechtigungen nicht. Ein Simple‑SSO‑Payload, der einen Benutzer mit diesen Berechtigungen benennt, erhält einen separaten Benutzer ohne Berechtigungen, und der ursprüngliche Benutzer bleibt unverändert.

Simple‑SSO‑Payloads sind nicht signiert, sodass bei aktivierter Option jeder, der die E‑Mail oder Benutzer‑ID eines Administrators kennt, als dieser handeln kann. Wenn das für Ihre Seite relevant ist, lassen Sie diese Option deaktiviert und verwenden Sie Secure SSO für Ihre Administratoren.

Benutzer, die durch Secure SSO oder die API erstellt wurden, können von Simple SSO nicht erreicht werden, unabhängig davon, ob diese Option aktiviert ist.

---