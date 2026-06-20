FastComments SDK ponuja tri API odjemalce:

### PublicAPI - Metode varne za uporabo na odjemalski strani

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). Te metode:
- Ne zahtevajo API key
- Lahko uporabljajo SSO tokene za overjanje
- So omejene glede hitrosti na uporabnika/napravo
- So primerne za aplikacije, namenjene končnim uporabnikom

**Primer uporabe**: Pridobivanje in ustvarjanje komentarjev v vaši iOS aplikaciji

### DefaultAPI - Metode na strežniški strani

The `DefaultAPI` contains authenticated methods that require an API key. Te metode:
- Zahtevajo vaš FastComments API key
- Naj se kličejo SAMO iz strežniške kode
- Omogočajo poln dostop do vaših FastComments podatkov
- So omejene glede hitrosti na tenant

**Primer uporabe**: Administrativne operacije, množični izvoz podatkov, upravljanje uporabnikov

### ModerationAPI - Metode nadzorne plošče moderatorja

The `ModerationAPI` contains methods that power the moderator dashboard. Te metode zajemajo:
- **Moderacija komentarjev** - seznam, štetje, iskanje, pridobivanje dnevnikov in izvoz komentarjev
- **Moderacijska dejanja** - odstranitev/obnova komentarjev, označevanje, nastavitev statusa pregleda/spama/odobritve, upravljanje glasov in ponovno odpiranje/zapiranje nití
- **Prepovedi** - prepoved uporabnika za komentar, razveljavitev prepovedi, pridobivanje povzetkov pred prepovedjo, preverjanje stanja prepovedi in nastavitev ter branje števila prepovedanih uporabnikov
- **Značke & zaupanje** - podeljevanje/odstranjevanje značk, izpis ročnih značk, pridobitev/nastavitev faktorja zaupanja uporabnika in branje notranjega profila uporabnika

Vsaka metoda v `ModerationAPI` sprejme parameter `sso`, tako da se lahko moderatorji avtenticirajo preko SSO.

**Primer uporabe**: Ustvarjanje moderacijske izkušnje za moderatorje vaše skupnosti

**POMEMBNO**: Nikoli ne razkrivajte vašega API key v odjemalski kodi. API keys naj se uporabljajo izključno na strežniški strani.