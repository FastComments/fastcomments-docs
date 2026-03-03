[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

За допомогою Simple SSO ми можемо надати віджету коментарів інформацію про користувача, щоб йому не потрібно було вводити своє ім'я користувача або електронну пошту для коментування.

Ми можемо налаштувати Simple SSO таким чином:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Користувач буде автоматично увійдений, і за лаштунками буде створено SSO User. У користувача буде встановлено `createdFromSimpleSSO` в `true`, якщо його отримано з API.

Notes: 

- Email is the unique identifier for Simple SSO.
- Providing an email with Simple SSO is not required, however by default their comments will show as "Unverified". <b>Якщо не вказано електронну пошту, користувача неможливо повністю автентифікувати.</b>
- **NEW** Since Jan 2022: Usernames do not have to be unique across all of fastcomments.com
- Simple SSO can automatically create and update SSO users, if an email is provided, and the user was not originally created from Secure SSO.
- Ви можете вказати значки для користувача за допомогою властивості `badgeConfig`. Масив `badgeIds` містить ідентифікатори глобальних значків, що пов'язуються з користувачем. Масив `pageBadgeIds` містить ідентифікатори значків, прив'язаних до поточної сторінки (`urlId`) — ці значки відображаються лише на сторінці, де їх призначено. Якщо `override` встановлено в `true`, це замінить існуючі відображувані значки (глобальні та призначені для сторінки перевизначаються незалежно); якщо `false`, то значки будуть додані до наявних.