[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Med Simple SSO kan vi give kommenterings-widgeten oplysninger om brugeren, så de ikke behøver at indtaste deres brugernavn eller e-mail for at kommentere.

Vi kan konfigurere Simple SSO som følger:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Brugeren vil blive logget ind, og vil oprette en SSO User bag kulisserne. Brugeren vil have `createdFromSimpleSSO` sat til `true` hvis hentet fra API'et.

Notes: 

- E-mail er den unikke identifikator for Simple SSO.
- Det er ikke nødvendigt at angive en e-mail med Simple SSO, men som standard vil deres kommentarer vises som "Uverificeret". <b>Hvis der ikke angives en e-mail, kan brugeren ikke blive fuldt autentificeret.</b>
- **NY** Siden Jan 2022: Brugernavne behøver ikke være unikke på tværs af hele fastcomments.com
- Simple SSO kan automatisk oprette og opdatere SSO-brugere, hvis en e-mail er angivet, og brugeren ikke oprindeligt blev oprettet fra Secure SSO.
- Du kan angive badges for brugeren med egenskaben `badgeConfig`. Arrayet `badgeIds` indeholder ID'erne for globale badges, der skal knyttes til brugeren. Arrayet `pageBadgeIds` indeholder badge-ID'er scoped til den aktuelle side (`urlId`) — disse badges vises kun på den side, hvor de blev tildelt. Hvis `override` er sat til `true`, vil det erstatte eksisterende viste badges (globale og side-scopeede overskrives uafhængigt); hvis `false` vil det blive lagt til de eksisterende badges.