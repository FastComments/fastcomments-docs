[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Mit Simple SSO können wir dem Kommentar-Widget Informationen über den Benutzer bereitstellen, sodass er seinen Benutzernamen oder seine E-Mail-Adresse nicht eingeben muss, um zu kommentieren.

Wir können Simple SSO wie folgt konfigurieren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Der Benutzer wird angemeldet, und im Hintergrund wird ein SSO-Benutzer erstellt. Der Benutzer hat `createdFromSimpleSSO` auf `true` gesetzt, wenn er über die API abgerufen wird.

Hinweise:

- Die E-Mail-Adresse ist der eindeutige Identifikator für Simple SSO.
- Die Angabe einer E-Mail-Adresse bei Simple SSO ist nicht erforderlich, allerdings werden die Kommentare standardmäßig als „Unverified" angezeigt. <b>Wenn keine E-Mail-Adresse angegeben wird, kann der Benutzer nicht vollständig authentifiziert werden.</b>
- **NEU** Seit Januar 2022: Benutzernamen müssen nicht mehr auf der gesamten fastcomments.com-Website eindeutig sein
- Simple SSO kann automatisch SSO-Benutzer erstellen und aktualisieren, wenn eine E-Mail-Adresse angegeben ist und der Benutzer nicht ursprünglich über Secure SSO erstellt wurde. Wenn ein Simple-SSO-Payload einen Benutzer nennt, der über Secure SSO oder die API erstellt wurde, bleibt dieser Benutzer unangetastet und der Besucher erhält einen separaten Benutzer.
- Sie können Abzeichen für den Benutzer über die Eigenschaft `badgeConfig` festlegen. Das Array `badgeIds` enthält die IDs der globalen Abzeichen, die dem Benutzer zugeordnet werden sollen. Das Array `pageBadgeIds` enthält Abzeichen-IDs, die auf die aktuelle Seite (`urlId`) beschränkt sind — diese Abzeichen werden nur auf der Seite angezeigt, auf der sie zugewiesen wurden. Wenn `override` auf `true` gesetzt ist, werden vorhandene angezeigte Abzeichen ersetzt (global und seitenspezifisch werden unabhängig voneinander überschrieben); bei `false` werden sie zu den vorhandenen Abzeichen hinzugefügt.

#### Simple-SSO-Einstellungen

Simple SSO ist immer verfügbar. Zwei Einstellungen steuern, wie viel FastComments ihm vertraut. Sie finden diese, indem Sie in einer <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">Anpassungsregel</a> nach SSO suchen. Beide sind standardmäßig deaktiviert und gelten unabhängig davon, ob „Require SSO To View Comments" aktiviert ist.

Konten, die diese Einstellungen bereits verwendeten, als sie hinzugefügt wurden, haben beide aktiviert, sodass sich für sie nichts geändert hat.

Nur die Anpassungsregel, die am besten zu einer Seite passt, wird angewendet. Wenn Sie also mehr als eine Regel haben, setzen Sie diese Einstellungen auf jeder davon. Eine neue Regel beginnt mit aktivierten Einstellungen, wenn eine Ihrer anderen Regeln sie aktiviert hat.

##### Kommentare über Simple SSO automatisch verifizieren

Wenn diese Option aktiviert ist, werden Kommentare von Simple-SSO-Benutzern sofort verifiziert.

Wenn diese Option deaktiviert ist, werden Kommentare von Simple-SSO-Benutzern als „Unverified" angezeigt, bis der Benutzer auf einen Link klickt, den wir an die Adresse im Simple-SSO-Payload senden. Wenn Sie nur verifizierte Kommentare automatisch freigeben, warten sie ebenfalls auf diesen Link. Wenn keine E-Mail-Adresse im Payload vorhanden ist, bleibt der Kommentar unverifiziert.

Diese Einstellung betrifft nur Kommentare. Abstimmungen von Simple-SSO-Benutzern werden immer sofort gezählt.

##### Simple-SSO-Administratoren zulassen

Wenn diese Option aktiviert ist, kann ein Simple-SSO-Benutzer Administrator oder Moderator Ihres Kontos sein:

- Ein Simple-SSO-Benutzer mit Administrator- oder Moderatorrechten behält diese.
- Ein Simple-SSO-Benutzer mit derselben E-Mail-Adresse wie einer Ihrer Administratoren oder Moderatoren übernimmt deren Rechte.

Wenn diese Option deaktiviert ist, gewährt Simple SSO diese Berechtigungen nicht. Ein Simple-SSO-Payload, der einen Benutzer mit solchen Rechten nennt, erhält einen separaten Benutzer ohne Berechtigungen, und der ursprüngliche Benutzer bleibt unangetastet.

Simple-SSO-Payloads sind nicht signiert. Wenn diese Option also aktiviert ist, kann jeder, der die E-Mail-Adresse oder Benutzer-ID eines Administrators kennt, sich als dieser ausgeben. Wenn das für Ihre Website relevant ist, lassen Sie diese Option deaktiviert und verwenden Sie Secure SSO für Ihre Administratoren.

Benutzer, die über Secure SSO oder die API erstellt wurden, können von Simple SSO nicht erreicht werden, unabhängig davon, ob diese Option aktiviert ist.