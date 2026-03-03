[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Са Simple SSO можемо да обезбедимо коментаришућем видгету информације о кориснику тако да не мора да уноси своје корисничко име или имејл да би коментарисао.

Simple SSO можемо конфигурисати на следећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Корисник ће бити пријављен, и иза сцене ће бити креиран SSO корисник. Поље `createdFromSimpleSSO` биће постављено на `true` ако је корисник добијен преко API-ја.

Напомене: 

- Е-пошта је јединствени идентификатор за Simple SSO.
- Пружање е-поште уз Simple SSO није обавезно, међутим по подразумеваној вредности њихови коментари ће се приказати као "Unverified". <b>Ако е-пошта није обезбеђена, корисник не може бити у потпуности аутентификован.</b>
- **NEW** Since Jan 2022: Usernames do not have to be unique across all of fastcomments.com
- Simple SSO може аутоматски да креира и ажурира SSO кориснике ако је е-пошта обезбеђена и корисник није оригинално креиран преко Secure SSO.
- Можете одредити значке за корисника помоћу `badgeConfig` својства. Низ `badgeIds` садржи ID-еве глобалних значки које ће бити повезане са корисником. Низ `pageBadgeIds` садржи ID-еве значки који су ограничени на тренутну страницу (`urlId`) — ове значке се приказују само на страници на којој су додељене. Ако је `override` постављен на `true`, он ће заменити постојеће приказане значке (глобалне и оне ограничене на страницу се потискују независно); ако је `false`, додаће се постојећим значкама.