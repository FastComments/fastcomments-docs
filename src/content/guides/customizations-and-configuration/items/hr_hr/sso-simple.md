---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Pomoću Simple SSO-a možemo pružiti widgetu za komentare informacije o korisniku tako da ne moraju unositi svoje korisničko ime ili e-poštu da bi komentirali.

Simple SSO možemo konfigurirati na sljedeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Korisnik će biti prijavljen i u pozadini će se stvoriti SSO korisnik. Korisnik će imati `createdFromSimpleSSO` postavljeno na `true` ako je dohvaćen iz API-ja.

Napomene: 

- E-pošta je jedinstveni identifikator za Simple SSO.
- Davanje e-pošte s Simple SSO-om nije obavezno, međutim, prema zadanim postavkama njihovi će komentari biti prikazani kao "Unverified". <b>Ako e-pošta nije navedena, korisnik se ne može u potpunosti autentificirati.</b>
- **NEW** Od siječnja 2022.: Korisnička imena ne moraju biti jedinstvena širom fastcomments.com
- Simple SSO može automatski stvarati i ažurirati SSO korisnike ako je e-pošta dostavljena i ako korisnik nije prvotno stvoren putem Secure SSO.
- Možete specificirati značke za korisnika koristeći svojstvo `badgeConfig`. Niz `badgeIds` sadrži ID-jeve znački koje će se povezati s korisnikom. Ako je `override` postavljen na `true`, zamijenit će sve postojeće značke prikazane na komentarima; ako je `false`, dodat će ih postojećim značkama.

---