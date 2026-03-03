[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

С помощью Simple SSO мы можем передать виджету комментариев информацию о пользователе, чтобы ему не приходилось вводить имя пользователя или адрес электронной почты для комментирования.

Simple SSO можно настроить следующим образом:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Пользователь будет авторизован, и в фоновом режиме будет создан SSO User. У пользователя будет свойство `createdFromSimpleSSO`, установленное в `true`, если он получен из API.

Notes: 

- Электронная почта является уникальным идентификатором для Simple SSO.
- Предоставление адреса электронной почты при использовании Simple SSO не обязательно, однако по умолчанию их комментарии будут отображаться как «Не подтверждено». <b>Если электронная почта не указана, пользователь не может быть полностью аутентифицирован.</b>
- **NEW** С января 2022 года: имена пользователей не обязаны быть уникальными на всём fastcomments.com
- Simple SSO может автоматически создавать и обновлять SSO пользователей, если указан адрес электронной почты и пользователь изначально не был создан с помощью Secure SSO.
- Вы можете указать значки для пользователя с помощью свойства `badgeConfig`. Массив `badgeIds` содержит ID глобальных значков, которые следует связать с пользователем. Массив `pageBadgeIds` содержит ID значков, привязанных к текущей странице (`urlId`) — эти значки отображаются только на странице, где они были назначены. Если `override` установлен в `true`, существующие отображаемые значки будут заменены (глобальные и значки, привязанные к странице, переопределяются независимо); если `false`, новые значки будут добавлены к существующим.