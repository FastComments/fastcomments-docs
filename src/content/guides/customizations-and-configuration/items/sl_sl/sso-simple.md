[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

S Simple SSO lahko komentarni vtičnik oskrbimo z informacijami o uporabniku, tako da mu ni treba vnašati uporabniškega imena ali e-poštnega naslova za komentiranje.

Simple SSO lahko konfiguriramo takole:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Uporabnik bo prijavljen in bo v ozadju ustvarjen SSO uporabnik. Če je uporabnik pridobljen prek API-ja, bo imel `createdFromSimpleSSO` nastavljen na `true`.

Opombe: 

- E-pošta je edinstveni identifikator za Simple SSO.
- Posredovanje e-poštnega naslova pri Simple SSO ni obvezno, vendar bodo njihovi komentarji privzeto prikazani kot "Nepreverjen". <b>Če e-poštni naslov ni podan, uporabnika ni mogoče popolnoma overiti.</b>
- **NOVO** Od januarja 2022: uporabniška imena ne morajo biti edinstvena na celotnem fastcomments.com
- Simple SSO lahko samodejno ustvari in posodobi SSO uporabnike, če je podan e-poštni naslov in če uporabnik ni bil prvotno ustvarjen z Secure SSO.
- Za uporabnika lahko določite značke z lastnostjo `badgeConfig`. Polje `badgeIds` vsebuje ID-je globalnih značk, ki jih je treba povezati z uporabnikom. Polje `pageBadgeIds` vsebuje ID-je značk omejene na trenutno stran (`urlId`) — te značke so prikazane samo na strani, kjer so bile dodeljene. Če je `override` nastavljeno na `true`, bo zamenjalo obstoječe prikazane značke (globalne in značke, omejene na stran, se prepišejo neodvisno); če je `false`, jih bo dodalo obstoječim značkam.