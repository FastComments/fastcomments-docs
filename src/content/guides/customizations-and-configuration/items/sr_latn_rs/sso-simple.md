[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Sa Simple SSO, možemo pružiti widgetu za komentarisanje informacije o korisniku tako da ne mora da unese svoje korisničko ime ili e‑mail da bi komentarisao.

Simple SSO možemo konfigurisati na sledeći način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Korisnik će biti prijavljen i kreiraće SSO korisnika u pozadini. Korisniku će biti postavljeno `createdFromSimpleSSO` na `true` ako je preuzet iz API‑ja.

Napomene: 

- E‑mail je jedinstveni identifikator za Simple SSO.
- Navođenje e‑maila uz Simple SSO nije obavezno, međutim po defaultu će njihovi komentari biti prikazani kao “Unverified”. <b>Ako e‑mail nije naveden, korisnik ne može biti potpuno autentifikovan.</b>
- **NOVO** Od januara 2022: Korisnička imena ne moraju biti jedinstvena na celom fastcomments.com
- Simple SSO može automatski kreirati i ažurirati SSO korisnike, ako je e‑mail naveden i korisnik nije prvobitno kreiran putem Secure SSO. Ako Simple SSO payload navodi korisnika kreiranog putem Secure SSO ili API‑ja, taj korisnik ostaje neizmenjen, a posetilac dobija zasebnog korisnika.
- Možete navesti značke (badges) za korisnika pomoću svojstva `badgeConfig`. Niz `badgeIds` sadrži ID‑ove globalnih znački koje se povezuju sa korisnikom. Niz `pageBadgeIds` sadrži ID‑ove znački ograničenih na trenutnu stranicu (`urlId`) — ove značke se prikazuju samo na stranici na kojoj su dodeljene. Ako je `override` postavljen na `true`, zamenjuje postojeće prikazane značke (globalne i po‑stranici se nezavisno prepisuju); ako je `false`, dodaje se postojećim znacima.

#### Podešavanja Simple SSO

Simple SSO je uvek dostupan. Dva podešavanja kontrolišu koliko FastComments mu veruje. Možete ih pronaći pretragom za SSO u <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagođavanja</a>. Obe su isključene po defaultu i primenjuju se bez obzira da li je “Require SSO To View Comments” uključen.

Nalozi koji su već koristili Simple SSO kada su ova podešavanja dodata imaju oba uključena, tako da se za njih ništa nije promenilo.

Samo pravilo prilagođavanja koje najbolje odgovara stranici se primenjuje, pa ako imate više pravila, postavite ova podešavanja na svako od njih. Novo pravilo počinje sa njima uključenim ako bilo koje od vaših drugih pravila ima ova podešavanja uključena.

##### Automatska verifikacija komentara putem Simple SSO

Kada je ovo uključeno, komentari od Simple SSO korisnika se odmah verifikuju.

Kada je ovo isključeno, komentari od Simple SSO korisnika se prikazuju kao “Unverified” dok korisnik ne klikne na link koji im pošaljemo e‑mailom na adresu iz Simple SSO payload‑a. Ako automatski odobravate samo verifikovane komentare, i oni čekaju na taj link. Ako u payload‑u nema e‑maila, komentar ostaje neverifikovan.

Ovo podešavanje utiče samo na komentare. Glasovi od Simple SSO korisnika se uvek odmah računaju.

##### Dozvoli Simple SSO administratore

Kada je ovo uključeno, Simple SSO korisnik može biti admin ili moderator vašeg naloga:

- Simple SSO korisnik koji ima admin ili moderator dozvole zadržava ih.
- Simple SSO korisnik sa istim e‑mailom kao jedan od vaših administratora ili moderatora preuzima njihove dozvole.

Kada je ovo isključeno, Simple SSO ne dodeljuje ove dozvole. Simple SSO payload koji navodi korisnika koji ih ima dobija zasebnog korisnika bez dozvola, a originalni korisnik ostaje neizmenjen.

Simple SSO payload‑i nisu potpisani, pa kada je ovo uključeno, bilo ko ko zna e‑mail ili ID administratora može se predstavljati kao on. Ako je to važno za vaš sajt, ostavite ovo isključeno i koristite Secure SSO za vaše administratore.

Korisnike kreirane putem Secure SSO ili API‑ja Simple SSO ne može dostići, bez obzira da li je ovo uključeno.