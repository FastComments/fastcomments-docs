[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Помоћу Simple SSO можемо доставити коментарском виџету информације о кориснику тако да не морају уносити своје корисничко име или е-пошту да би коментарисали.

Simple SSO можемо конфигурисати на следећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Корисник ће бити пријављен, и иза сцене ће бити креиран SSO корисник. Кориснику ће `createdFromSimpleSSO` бити постављено на `true` ако је преузет из API-ја.

Notes: 

- Е-пошта је јединствени идентификатор за Simple SSO.
- Давање е-поште уз Simple SSO није обавезно, међутим по подразумеваној вредности њихови коментари ће бити приказани као „Непотврђено“. <b>Ако није достављена е-пошта, корисник не може бити потпуно аутентификован.</b>
- **НОВО** Од јануара 2022: Корисничка имена не морају бити јединствена на целој fastcomments.com
- Simple SSO може аутоматски креирати и ажурирати SSO кориснике, ако је достављена е-пошта и ако корисник у почетку није креиран преко Secure SSO.
- Можете назначити значке за корисника помоћу `badgeConfig` својства. Низ `badgeIds` садржи ID-еве значки које ће се повезати са корисником. Ако је `override` постављен на `true`, то ће заменити све постојеће значке приказане на коментарима; ако је `false`, додаће се постојећим значкама.