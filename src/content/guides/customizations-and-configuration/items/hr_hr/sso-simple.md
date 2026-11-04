[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Uz Simple SSO možemo widgetu za komentiranje pružiti informacije o korisniku kako ne bi morao unositi svoje korisničko ime ili e‑mail za komentiranje.

Simple SSO možemo konfigurirati na sljedeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Jednostavni SSO'; code-example-end]

Korisnik će biti prijavljen i u pozadini će se stvoriti SSO korisnik. Korisnik će imati `createdFromSimpleSSO` postavljen na `true` ako je dohvaćen iz API-ja.

Napomene:

- E‑mail je jedinstveni identifikator za Simple SSO.
- Navođenje e‑maila uz Simple SSO nije obavezno, međutim prema zadanim postavkama njihovi će komentari biti prikazani kao "Neprovjereni". <b>Ako e‑mail nije naveden, korisnik se ne može potpuno autentificirati.</b>
- **NOVO** Od siječnja 2022.: Korisnička imena ne moraju biti jedinstvena na cijelom fastcomments.com
- Simple SSO može automatski stvoriti i ažurirati SSO korisnike, ako je e‑mail naveden i korisnik nije izvorno stvoren putem Secure SSO. Ako Simple SSO payload navodi korisnika koji je kreiran putem Secure SSO ili API-ja, taj korisnik ostaje nepromijenjen, a posjetitelj dobiva zasebnog korisnika.
- Možete specificirati značke za korisnika pomoću svojstva `badgeConfig`. Polje `badgeIds` sadrži ID‑ove globalnih znački koje se povezuju s korisnikom. Polje `pageBadgeIds` sadrži ID‑ove znački ograničenih na trenutnu stranicu (`urlId`) — te se značke prikazuju samo na stranici na kojoj su dodijeljene. Ako je `override` postavljen na `true`, zamijenit će postojeće prikazane značke (globalne i stranicom‑ograničene se zamjenjuju neovisno); ako je `false`, dodati će se uz postojeće značke.

#### Postavke Simple SSO

Simple SSO je uvijek dostupan. Dvije postavke kontroliraju koliko FastComments mu vjeruje. Možete ih pronaći pretraživanjem SSO u <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagodbe</a>. Obje su prema zadanim postavkama isključene i primjenjuju se bez obzira je li opcija "Zahtijevaj SSO za pregled komentara" uključena.

Računi koji su već koristili Simple SSO kada su ove postavke dodane imaju obje uključene, pa se ništa nije promijenilo za njih.

Primjenjuje se samo pravilo prilagodbe koje najbolje odgovara stranici, pa ako imate više pravila, postavite ih na svako od njih. Novo pravilo započinje s njima uključenim ako bilo koje od vaših drugih pravila ima uključene.

##### Automatska provjera komentara putem Simple SSO

Kada je ovo uključeno, komentari od Simple SSO korisnika se odmah verificiraju.

Kada je ovo isključeno, komentari od Simple SSO korisnika prikazuju se kao "Neprovjereni" dok korisnik ne klikne na poveznicu koju im pošaljemo e‑mailom na adresu iz Simple SSO payloada. Ako automatski odobravate samo verificirane komentare, i oni čekaju na tu poveznicu. Ako u payloadu nema e‑maila, komentar ostaje neprovjeren.

Ova postavka utječe samo na komentare. Glasovi od Simple SSO korisnika uvijek se odmah računaju.

##### Dozvoli Simple SSO administratore

Kada je ovo uključeno, Simple SSO korisnik može biti administrator ili moderator vašeg računa:

- Simple SSO korisnik koji ima administratorske ili moderatorske dozvole zadržava ih.
- Simple SSO korisnik s istim e‑mailom kao jedan od vaših administratora ili moderatora preuzima njihove dozvole.

Kada je ovo isključeno, Simple SSO ne dodjeljuje ove dozvole. Simple SSO payload koji navodi korisnika s tim dozvolama dobiva zasebnog korisnika bez dozvola, a izvorni korisnik ostaje nepromijenjen.

Simple SSO payloadi nisu potpisani, pa kada je ovo uključeno, bilo tko tko zna e‑mail administratora ili ID korisnika može se predstavljati kao on. Ako je to važno za vašu stranicu, ostavite ovo isključeno i koristite Secure SSO za svoje administratore.

Korisnike koje je stvorio Secure SSO ili API Simple SSO ne može dosegnuti, bez obzira je li ovo uključeno.