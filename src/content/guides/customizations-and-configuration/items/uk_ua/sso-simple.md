[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

За допомогою Simple SSO ми можемо надати віджетові коментування інформацію про користувача, щоб йому не довелося вводити ім'я користувача або електронну пошту для коментування.

Ми можемо сконфігурувати Simple SSO таким чином:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Користувач увійде в систему, і за лаштунками буде створено SSO-користувача. У користувача буде встановлено `createdFromSimpleSSO` в значення `true`, якщо його отримано з API.

Notes: 

- Електронна пошта є унікальним ідентифікатором для Simple SSO.
- Надання електронної пошти для Simple SSO не є обов'язковим, однак за замовчуванням їхні коментарі будуть показані як "Не перевірено". <b>Якщо електронна пошта не надана, користувача неможливо повністю автентифікувати.</b>
- **НОВЕ** З січня 2022: Імена користувачів не мають бути унікальними по всьому fastcomments.com
- Simple SSO може автоматично створювати та оновлювати SSO-користувачів, якщо вказано електронну пошту, і якщо користувач не був спочатку створений через Secure SSO.
- Ви можете задати бейджі для користувача за допомогою властивості `badgeConfig`. Масив `badgeIds` містить ID бейджів, які потрібно пов'язати з користувачем. Якщо `override` встановлено в `true`, це замінить усі існуючі бейджі, що відображаються в коментарях; якщо `false` — додасться до існуючих бейджів.

---