[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Med Simple SSO kan vi give kommentarfunktionen information om brugeren, så de ikke behøver at indtaste brugernavn eller e‑mail for at kommentere.

Vi kan konfigurere Simple SSO som følger:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Brugeren vil være logget ind og vil bag kulisserne oprette en SSO‑bruger. Brugeren vil have `createdFromSimpleSSO` sat til `true`, hvis den hentes fra API‑et.

Bemærk:

- E‑mail er den unikke identifikator for Simple SSO.
- Det er ikke påkrævet at angive en e‑mail med Simple SSO, men som standard vil deres kommentarer vises som "Ubekræftet". <b>Hvis der ikke angives e‑mail, kan brugeren ikke blive fuldt autentificeret.</b>
- **NY** Siden januar 2022: Brugernavne behøver ikke være unikke på tværs af hele fastcomments.com
- Simple SSO kan automatisk oprette og opdatere SSO‑brugere, hvis en e‑mail er angivet, og brugeren ikke oprindeligt blev oprettet fra Secure SSO. Hvis en Simple SSO‑payload navngiver en bruger, der er oprettet af Secure SSO eller API‑et, forbliver den bruger urørt, og besøgende får en separat bruger.
- Du kan angive badges for brugeren med `badgeConfig`‑egenskaben. `badgeIds`‑arrayet indeholder ID‑erne på globale badges, der skal knyttes til brugeren. `pageBadgeIds`‑arrayet indeholder badge‑ID‑er, der er scoped til den aktuelle side (`urlId`) — disse badges vises kun på den side, hvor de blev tildelt. Hvis `override` er sat til `true`, erstattes eksisterende viste badges (globale og side‑scoped overskrives uafhængigt); hvis `false`, tilføjes de til eksisterende badges.

#### Simple SSO-indstillinger

Simple SSO er altid tilgængelig. To indstillinger styrer, hvor meget FastComments stoler på den. Du kan finde dem ved at søge efter SSO i en <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">tilpasningsregel</a>. Begge er deaktiveret som standard, og de gælder uanset om "Kræv SSO for at se kommentarer" er slået til.

Konti, der allerede brugte Simple SSO, da disse indstillinger blev tilføjet, har begge slået til, så intet ændrede sig for dem.

Kun den tilpasningsregel, der bedst matcher en side, anvendes, så hvis du har mere end én regel, skal du sætte disse på hver af dem. En ny regel starter med dem slået til, hvis en af dine andre regler har dem slået til.

##### Auto‑bekræft kommentarer via Simple SSO

Når dette er slået til, bekræftes kommentarer fra Simple SSO‑brugere med det samme.

Når dette er slået fra, vises kommentarer fra Simple SSO‑brugere som "Ubekræftet", indtil brugeren klikker på et link, vi sender til adressen i Simple SSO‑payloaden. Hvis du kun automatisk godkender bekræftede kommentarer, venter de også på dette link. Hvis der ikke er nogen e‑mail i payloaden, forbliver kommentaren ubekræftet.

Denne indstilling påvirker kun kommentarer. Stemmer fra Simple SSO‑brugere tælles altid med det samme.

##### Tillad Simple SSO-administratorer

Når dette er slået til, kan en Simple SSO‑bruger være administrator eller moderator af din konto:

- En Simple SSO‑bruger, der har administrator‑ eller moderatorrettigheder, beholder dem.
- En Simple SSO‑bruger med samme e‑mail som en af dine administratorer eller moderatorer overtager deres rettigheder.

Når dette er slået fra, giver Simple SSO ikke disse rettigheder. En Simple SSO‑payload, der navngiver en bruger med disse rettigheder, får en separat bruger uden rettigheder, og den oprindelige bruger forbliver urørt.

Simple SSO‑payloads er ikke signeret, så når dette er slået til, kan enhver, der kender en administrators e‑mail eller bruger‑ID, handle som dem. Hvis det er vigtigt for dit site, så lad dette være slået fra og brug Secure SSO til dine administratorer.

Brugere oprettet af Secure SSO eller API‑et kan ikke nås af Simple SSO, uanset om dette er slået til eller ej.