[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Uz Simple SSO možemo pružiti widgetu za komentiranje informacije o korisniku kako ne bi morali unositi svoje korisničko ime ili e‑mail za komentiranje.

Simple SSO možemo konfigurirati na sljedeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Jednostavni SSO'; code-example-end]

Korisnik će biti prijavljen i u pozadini će se stvoriti SSO korisnik. Korisnik će imati `createdFromSimpleSSO` postavljeno na `true` ako je dohvaćen putem API‑ja.

Napomene: 

- E‑mail je jedinstveni identifikator za Simple SSO.
- Navođenje e‑maila uz Simple SSO nije obavezno, međutim prema zadanim postavkama njihovi će komentari biti prikazani kao "Unverified". <b>Ako nije naveden e‑mail, korisnik se ne može potpuno autentificirati.</b>
- **NOVO** Od siječnja 2022.: Korisnička imena ne moraju biti jedinstvena na cijelom fastcomments.com
- Simple SSO može automatski stvoriti i ažurirati SSO korisnike, ako je e‑mail naveden i korisnik nije izvorno stvoren putem Secure SSO.
- Možete odrediti značke za korisnika pomoću svojstva `badgeConfig`. Polje `badgeIds` sadrži ID‑ove globalnih znački koje se povezuju s korisnikom. Polje `pageBadgeIds` sadrži ID‑ove znački ograničenih na trenutnu stranicu (`urlId`) — ove značke se prikazuju samo na stranici na kojoj su dodijeljene. Ako je `override` postavljeno na `true`, zamijenit će postojeće prikazane značke (globalne i na stranici ograničene značke zamjenjuju se neovisno); ako je `false`, dodati će se uz postojeće značke.