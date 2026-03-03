[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Са Simple SSO-ом, можемо пружити коментарском видџету информације о кориснику тако да не морају да уносе своје корисничко име или имејл да би коментарисали.

Simple SSO можемо конфигурисати на следећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Корисник ће бити пријављен, и иза сцене ће бити креиран SSO User. Корисник ће имати `createdFromSimpleSSO` постављено на `true` ако је преузет из API-ја.

Напомене: 

- Е-пошта је јединствени идентификатор за Simple SSO.
- Навођење е-поште уз Simple SSO није обавезно, међутим по подразумевaњу њихови коментари ће бити приказани као "Unverified". <b>Ако е-пошта није наведена, корисник не може бити у потпуности аутентификован.</b>
- **NEW** Од јануара 2022: Корисничка имена не морају бити јединствена на целој fastcomments.com
- Simple SSO аутоматски може креирати и ажурирати SSO кориснике, ако је наведена е-пошта, и ако корисник није изворно креиран преко Secure SSO.
- Можете одредити значке за корисника помоћу својства `badgeConfig`. Низ `badgeIds` садржи ID-еве глобалних значки које ће бити повезане са корисником. Низ `pageBadgeIds` садржи ID-еве значки који су ограничени на тренутну страницу (`urlId`) — ове значке се приказују само на страници где су додељене. Ако је `override` постављено на `true`, то ће заменити постојеће приказане значке (глобалне и странице ограничене значке се замењују независно); ако је `false` додаће се на постојеће значке.

---