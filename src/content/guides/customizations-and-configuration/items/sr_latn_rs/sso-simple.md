[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Sa Simple SSO-om, možemo obezbediti widgetu za komentare informacije o korisniku tako da ne moraju da unose svoje korisničko ime ili email da bi komentarisali.

Možemo konfigurisati Simple SSO na sledeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Korisnik će biti prijavljen, i biće kreiran SSO User u pozadini. Korisnik će imati `createdFromSimpleSSO` postavljeno na `true` ako je dohvaćen iz API-ja.

Notes: 

- Email je jedinstveni identifikator za Simple SSO.
- Navođenje email adrese uz Simple SSO nije obavezno, međutim po defaultu njihovi komentari će se prikazivati kao "Unverified". <b>Ako nije obezbeđen email, korisnik ne može biti u potpunosti autentifikovan.</b>
- **NOVO** Od januara 2022: Korisnička imena ne moraju biti jedinstvena na celom fastcomments.com
- Simple SSO može automatski da kreira i ažurira SSO korisnike, ako je email obezbeđen, i ako korisnik nije originalno kreiran preko Secure SSO.
- Možete odrediti bedževe za korisnika pomoću svojstva `badgeConfig`. Niz `badgeIds` sadrži ID-jeve bedževa koji će biti povezani sa korisnikom. Ako je `override` postavljen na `true`, zamenit će sve postojeće bedževe prikazane na komentarima; ako je `false`, dodaće se postojećim bedževima.