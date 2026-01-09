[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Помоћу Simple SSO-а можемо дати коментарском видгету информације о кориснику тако да не морају да уносе корисничко име или имејл да би коментарисали.

Simple SSO се може конфигурисати на следећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Корисник ће бити пријављен, и у позадини ће бити креиран SSO корисник. Кориснику ће својство `createdFromSimpleSSO` бити постављено на `true` ако се повуче преко API-ја.

Напомене: 

- Е-пошта је јединствени идентификатор за Simple SSO.
- Није обавезно доставити е-пошту уз Simple SSO, међутим по подразумеваној вредности њихови коментари ће се приказивати као "Непотврђено". <b>Ако није достављена е-пошта, корисник не може бити потпуно аутентификован.</b>
- **NEW** Од јануара 2022: Корисничка имена не морају бити јединствена у оквиру целог fastcomments.com
- Simple SSO може аутоматски да креира и ажурира SSO кориснике ако је обезбеђена е-пошта, и ако корисник није првобитно креиран помоћу Secure SSO.
- Можете назначити значке за корисника помоћу својства `badgeConfig`. Низ `badgeIds` садржи ИД-ове значки које треба повезати са корисником. Ако је `override` постављен на `true`, то ће заменити све постојеће значке које се приказују на коментарима; ако је `false`, додаће их постојећим значкама.