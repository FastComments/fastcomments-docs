[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Sa Simple SSO možemo obezbediti widgetu za komentare informacije o korisniku tako da ne moraju unositi svoje korisničko ime ili email da bi komentarisali.

Simple SSO možemo konfigurisati na sledeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Korisnik će biti prijavljen, i biće kreiran SSO korisnik u pozadini. Korisnik će imati `createdFromSimpleSSO` postavljeno na `true` ako je preuzet iz API-ja.

Notes: 

- Email je jedinstveni identifikator za Simple SSO.
- Navođenje email-a za Simple SSO nije obavezno, međutim podrazumevano će njihovi komentari biti prikazani kao "Unverified". <b>Ako nije naveden email, korisnik ne može biti potpuno autentifikovan.</b>
- **NEW** Od jan 2022: Korisnička imena ne moraju biti jedinstvena na celom fastcomments.com
- Simple SSO može automatski kreirati i ažurirati SSO korisnike, ako je email naveden, i ako korisnik nije originalno kreiran putem Secure SSO.
- Možete navesti značke za korisnika pomoću svojstva `badgeConfig`. Niz `badgeIds` sadrži ID-jeve globalnih znački koje će se povezati sa korisnikom. Niz `pageBadgeIds` sadrži ID-jeve znački ograničene na tekuću stranu (`urlId`) — ove značke se prikazuju samo na stranici na kojoj su dodeljene. Ako je `override` postavljeno na `true`, zameniće postojeće prikazane značke (globalne i one ograničene na stranu se zamenjuju nezavisno); ako je `false`, dodaće se postojećim značkama.

---