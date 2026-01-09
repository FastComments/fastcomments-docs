[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Z Simple SSO lahko pripomočku za komentiranje zagotovimo informacije o uporabniku, tako da mu ni treba vnesti svojega uporabniškega imena ali e-pošte za komentiranje.

Simple SSO lahko konfiguriramo tako:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Uporabnik bo prijavljen in bo za kulisami ustvarjen SSO uporabnik. Lastnost `createdFromSimpleSSO` bo nastavljena na `true`, če je uporabnik pridobljen prek API-ja.

Opombe: 

- E-pošta je edinstveni identifikator za Simple SSO.
- Podajanje e-pošte pri Simple SSO ni obvezno, vendar bodo njihovi komentarji privzeto prikazani kot "Neoverjeni". <b>Če e-pošta ni podana, uporabnik ne more biti popolnoma overjen.</b>
- **NOVO** Od januarja 2022: uporabniška imena niso več nujno edinstvena na celotnem fastcomments.com
- Simple SSO lahko samodejno ustvari in posodobi SSO uporabnike, če je podana e-pošta in če uporabnik ni bil prvotno ustvarjen prek Secure SSO.
- Za uporabnika lahko določite značke s lastnostjo `badgeConfig`. Polje `badgeIds` vsebuje ID-je značk, ki jih je treba povezati z uporabnikom. Če je `override` nastavljeno na `true`, bo nadomestilo vse obstoječe značke, prikazane pri komentarjih; če je `false`, jih bo dodalo k obstoječim značkam.