---
Pokreće se kada moderator dodeli značku korisniku.

### Kontekst koji agent prima

- **ID značke** koja je dodeljena.
- **ID korisnika koji je pokrenuo okidač** - moderator koji je dodelio značku.
- Opcionalni kontekst niti / istorija korisnika / kontekst stranice prema konfiguraciji.

Podaci okidača **ne** uključuju `commentId` u podacima okidača, čak i ako je značka dodeljena u vezi sa određenim komentarom.

### Ko pokreće ovo

Radnja ljudskog moderatora.

### Napomene

- Uključuje se samo ID značke; agent ne prima metapodatke značke (naziv, slika). Ako agentu treba da utvrdi *koja* značka je dodeljena, ugrađite taj kontekst u [početni upit](#personality-prompt) ili u [smernice zajednice](#community-guidelines).
- Okidač se aktivira jednom po dodeli značke, a ne po korisniku. Dodeljivanje iste značke korisniku dva puta aktivira ga dva puta (svaka dodela je poseban događaj).

### Uobičajene upotrebe

- **Recipročno priznanje** - agent može objaviti odgovor "hvala na sjajnom doprinosu" kada je dodeljena određena značka.
- **Spoljni tok za prepoznavanje** putem [Webhooks](#webhooks-overview) - preslikajte dodele znački u sopstveni sistem za angažman korisnika.
- **Zabeležavanje u memoriju** - beleške "ovaj korisnik je prepoznat kao saradnik" koje bi budući agenti za moderaciju trebali uzeti u obzir pri donošenju odluka.

---