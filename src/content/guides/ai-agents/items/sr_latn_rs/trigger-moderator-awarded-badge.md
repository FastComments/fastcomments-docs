---
Okida se kada moderator dodeli značku korisniku.

### Kontekst koji agent prima

- **ID značke** dodeljene značke.
- **ID korisnika koji je okidač** - moderator koji je dodelio značku.
- Opcionalni kontekst teme / istorije korisnika / stranice prema konfiguraciji.

Mesto okidanja ne uključuje `commentId` u payload-u okidača, čak i ako je značka dodeljena u vezi sa određenim komentarom.

### Ko ovo okida

Akcija ljudskog moderatora.

### Napomene

- U payload je uključen samo ID značke; agent ne dobija metapodatke značke (ime, sliku). Ako agent treba da razume *koja* je značka dodeljena, ugrađujte taj kontekst u [početni prompt](#personality-prompt) ili [smernice zajednice](#community-guidelines).
- Okidač se pokreće jednom po dodeli značke, ne po korisniku. Dodeljivanje iste značke korisniku dva puta okinuće ga dva puta (svaka dodela je poseban događaj).

### Uobičajene upotrebe

- **Recipročna priznanja** - agent može objaviti odgovor "hvala za odličan doprinos" kada je određena značka dodeljena.
- **Spoljni tok priznanja** putem [Webhooks](#webhooks-overview) - preslikajte dodele znački u sopstveni sistem angažovanja korisnika.
- **Zabeležavanje memorije** - beleške poput "ovaj korisnik je prepoznat kao saradnik" koje budući moderacijski agenti treba da uzmu u obzir pri donošenju odluka.

---