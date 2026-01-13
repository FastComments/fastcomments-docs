---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Dzięki Simple SSO możemy dostarczyć widgetowi komentarzy informacje o użytkowniku, aby nie musiał podawać swojej nazwy użytkownika ani adresu e-mail, aby skomentować.

Możemy skonfigurować Simple SSO w następujący sposób:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Użytkownik zostanie zalogowany i za kulisami zostanie utworzony użytkownik SSO. Użytkownik będzie miał ustawione `createdFromSimpleSSO` na `true`, jeśli zostanie pobrany z API.

Notes: 

- E-mail jest unikalnym identyfikatorem dla Simple SSO.
- Podanie adresu e-mail przy Simple SSO nie jest wymagane, jednak domyślnie ich komentarze będą wyświetlane jako "Niezweryfikowany". <b>Jeśli nie zostanie podany adres e-mail, użytkownik nie może zostać w pełni uwierzytelniony.</b>
- **NOWOŚĆ** Od stycznia 2022: Nazwy użytkowników nie muszą być unikalne w całym fastcomments.com
- Simple SSO może automatycznie tworzyć i aktualizować użytkowników SSO, jeśli podany jest adres e-mail i użytkownik nie został pierwotnie utworzony przez Secure SSO.
- Możesz określić odznaki dla użytkownika za pomocą właściwości `badgeConfig`. Tablica `badgeIds` zawiera identyfikatory odznak, które mają być powiązane z użytkownikiem. Jeśli `override` jest ustawione na `true`, zastąpi wszystkie istniejące odznaki wyświetlane przy komentarzach; jeśli `false`, doda je do istniejących odznak.

---