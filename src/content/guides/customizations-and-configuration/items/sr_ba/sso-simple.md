[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Са Simple SSO можемо обезбиједити коментарски видгет информацијама о кориснику тако да не морају уносити своје корисничко име или е-пошту да би коментарисали.

Simple SSO можемо конфигурисати на сљедећи начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Корисник ће бити пријављен и иза сцене ће се креирати SSO корисник. Кориснику ће својство `createdFromSimpleSSO` бити постављено на `true` ако је преузет преко API-ја.

Notes: 

- Е-пошта је јединствени идентификатор за Simple SSO.
- Обезбјеђивање е-поште уз Simple SSO није обавезно, међутим по подразумјеваној поставци њихови коментари ће се приказивати као "Unverified". <b>Ако е-пошта није достављена, корисник не може бити у потпуности аутентификован.</b>
- **NEW** Од јануара 2022: Корисничка имена не морају бити јединствена на целом fastcomments.com
- Simple SSO може аутоматски креирати и ажурирати SSO кориснике ако је обезбијеђена е-пошта и ако корисник није првобитно креиран преко Secure SSO.
- Можете назначити значке за корисника помоћу својства `badgeConfig`. Низ `badgeIds` садржи ID-еве значки које треба повезати са корисником. Ако је `override` постављен на `true`, то ће замјенити све постојеће значке приказане на коментарима; ако је `false`, додаваће се постојећим значкама.