Pokreće se kada moderator dodijeli značku korisniku.

### Kontekst koji agent prima

- **ID značke** dodijeljene.
- **ID korisnika koji je pokrenuo događaj** - moderator koji je dodijelio značku.
- Opcionalni kontekst niti / povijesti korisnika / stranice prema konfiguraciji.

Mjesto okidanja ne uključuje `commentId` u payloadu okidača, čak i ako je značka dodijeljena u vezi s određenim komentarom.

### Tko pokreće ovo

Akcija ljudskog moderatora.

### Važno

- Uključuje se samo ID značke; agent ne prima metapodatke značke (naziv, slika). Ako agentu treba razmišljati o *kojoj* znački je riječ, ugrađujte taj kontekst u [početni upit](#personality-prompt) ili [smjernice zajednice](#community-guidelines).
- Okidač se aktivira jednom po dodjeli značke, ne po korisniku. Dodjela iste značke korisniku dvaput aktivira okidač dvaput (svaka dodjela je zaseban događaj).

### Uobičajene upotrebe

- **Međusobno priznanje** - agent može objaviti odgovor "hvala na sjajnom doprinosu" kada je dodijeljena određena značka.
- **Vanjski tijek rada priznavanja** preko [Webhooks](#webhooks-overview) - preslikajte dodjele znački u vlastiti sustav za angažman korisnika.
- **Zapis u memoriji** - bilješke poput "ovaj korisnik je priznati suradnik" koje bi budući moderacijski agenti trebali uzeti u obzir pri donošenju odluka.

---