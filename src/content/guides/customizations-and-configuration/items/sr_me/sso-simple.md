[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Sa Simple SSO можемо обезбиједити widget за коментаре информацијама о кориснику, тако да не морају уносити своје корисничко име или е‑маил да би коментарисали.

Simple SSO можемо конфигурисати на следећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Кorisnik ће бити пријављен, и у позадини ће бити креиран SSO корисник. Корисник ће имати `createdFromSimpleSSO` постављено на `true` ако је дохваћен из API‑ја.

Напомене: 

- E-mail је једinstвени идентификатор за Simple SSO.
- Нavoђење e‑mail адресе код Simple SSO није обавезно, међутим, по подразумеваној поставци њихови коментари ће се приказивати као "Unverified". <b>Aко e‑mail није наведен, корисник не може бити у потпуности аутентификован.</b>
- **NEW** Од јануара 2022: Корисничка имена не морају бити јединствена на целој платформи fastcomments.com
- Simple SSO може аутоматски креирати и ажурирати SSO кориснике, ако је е‑mail наведен и ако корисник није првобитно креиран преко Secure SSO.
- Можете назначити ознаке (badges) за корисника помоћу особине `badgeConfig`. Низ `badgeIds` садржи ID‑је глобалних значки које треба повезати са корисником. Низ `pageBadgeIds` садржи ID‑је значки које су ограничене на тренутну страницу (`urlId`) — те значке се приказују само на страници на којој су додељене. Ако је `override` постављено на `true`, то ће заменити постојеће приказане значке (глобалне и значке ограничене на страницу се мењају независно); ако је `false`, додаће се на постојеће значке.

---