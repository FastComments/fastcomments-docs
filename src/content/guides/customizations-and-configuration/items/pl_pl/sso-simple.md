[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Dzięki Simple SSO możemy przekazać widgetowi komentarzy informacje o użytkowniku, aby nie musiał on podawać swojej nazwy użytkownika ani adresu e-mail, aby dodać komentarz.

Możemy skonfigurować Simple SSO w następujący sposób:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Użytkownik zostanie zalogowany, a w tle zostanie utworzony użytkownik SSO. Użytkownik będzie miał ustawione `createdFromSimpleSSO` na `true`, jeśli zostanie pobrany z API.

Notes: 

- Adres e-mail jest unikalnym identyfikatorem dla Simple SSO.
- Podanie adresu e-mail przy użyciu Simple SSO nie jest wymagane, jednak domyślnie ich komentarze będą wyświetlane jako "Nieweryfikowany". <b>Jeśli nie podano adresu e-mail, użytkownik nie może być w pełni uwierzytelniony.</b>
- **NOWE** Od stycznia 2022: Nazwy użytkowników nie muszą być unikalne w całym serwisie fastcomments.com
- Simple SSO może automatycznie tworzyć i aktualizować użytkowników SSO, jeśli podano adres e-mail, a użytkownik nie został pierwotnie utworzony za pomocą Secure SSO.
- Możesz określić odznaki dla użytkownika za pomocą właściwości `badgeConfig`. Tablica `badgeIds` zawiera identyfikatory globalnych odznak powiązanych z użytkownikiem. Tablica `pageBadgeIds` zawiera identyfikatory odznak przypisanych do bieżącej strony (`urlId`) — te odznaki są wyświetlane tylko na stronie, na której zostały przypisane. Jeśli `override` jest ustawione na `true`, zastąpi istniejące wyświetlane odznaki (globalne i przypisane do strony są zastępowane niezależnie); jeśli `false`, zostaną dodane do istniejących odznak.

---