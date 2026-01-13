[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

С помощью Simple SSO мы можем предоставить виджету комментариев информацию о пользователе, чтобы ему не нужно было вводить имя пользователя или электронную почту для комментирования.

Мы можем настроить Simple SSO следующим образом:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Пользователь будет авторизован, и за кулисами будет создан SSO-пользователь. У пользователя будет поле `createdFromSimpleSSO`, установленное в `true`, если он получен через API.

Notes: 

- Email является уникальным идентификатором для Simple SSO.
- Предоставление электронной почты при использовании Simple SSO не обязательно, однако по умолчанию их комментарии будут отображаться как "Unverified". <b>Если электронная почта не указана, пользователь не может быть полностью аутентифицирован.</b>
- **НОВОЕ** С января 2022: имена пользователей больше не обязаны быть уникальными для всего fastcomments.com
- Simple SSO может автоматически создавать и обновлять SSO-пользователей, если указана электронная почта и пользователь изначально не был создан через Secure SSO.
- Вы можете указать бейджи для пользователя через свойство `badgeConfig`. Массив `badgeIds` содержит идентификаторы бейджей, которые следует связать с пользователем. Если `override` установлено в `true`, это заменит все существующие бейджи, отображаемые в комментариях; если в `false` — добавит к существующим бейджам.

---