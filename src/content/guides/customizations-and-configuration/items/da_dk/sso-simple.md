[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Med Simple SSO kan vi give kommenteringswidget'en oplysninger om brugeren, så de ikke behøver at indtaste deres brugernavn eller e-mail for at kommentere.

Vi kan konfigurere Simple SSO som følger:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Brugeren vil blive logget ind, og der oprettes en SSO-bruger bag kulisserne. Brugeren vil have `createdFromSimpleSSO` sat til `true` hvis den hentes fra API'en.

Noter: 

- E-mail er den unikke identifikator for Simple SSO.
- Det er ikke påkrævet at angive en e-mail ved Simple SSO; som standard vil deres kommentarer dog vises som "Uverificeret". <b>Hvis der ikke angives en e-mail, kan brugeren ikke blive fuldt autentificeret.</b>
- **NYT** Siden Jan 2022: Brugernavne behøver ikke være unikke på tværs af hele fastcomments.com
- Simple SSO kan automatisk oprette og opdatere SSO-brugere, hvis der angives en e-mail, og brugeren ikke oprindeligt blev oprettet fra Secure SSO.
- Du kan angive badges for brugeren med egenskaben `badgeConfig`. Arrayet `badgeIds` indeholder ID'erne for badges, der skal tilknyttes brugeren. Hvis `override` er sat til `true`, vil det erstatte alle eksisterende badges, der vises på kommentarer; hvis `false` vil det blive føjet til de eksisterende badges.