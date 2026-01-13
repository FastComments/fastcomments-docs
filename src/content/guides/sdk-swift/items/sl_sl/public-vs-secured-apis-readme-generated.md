The FastComments SDK zagotavlja dve vrsti API končnih točk:

### `PublicAPI` - Končne točke varne za odjemalca

The `PublicAPI` vsebuje končne točke, ki jih je varno klicati iz odjemalčeve kode (aplikacije iOS/macOS). Te končne točke:
- Ne zahtevajo API key
- Lahko uporabljajo SSO tokens za avtentikacijo
- So omejene glede na število zahtevkov na uporabnika/napravo
- So primerne za aplikacije, namenjene končnim uporabnikom

**Primer uporabe**: Pridobivanje in ustvarjanje komentarjev v vaši iOS aplikaciji

### `DefaultAPI` - Končne točke na strežniški strani

The `DefaultAPI` vsebuje avtenticirane končne točke, ki zahtevajo API key. Te končne točke:
- Zahtevajo vaš FastComments API key
- Jih SMETE klicati SAMO iz strežniške kode
- Omogočajo popoln dostop do vaših FastComments podatkov
- So omejene glede na število zahtevkov na najemnika

**Primer uporabe**: Administrativne operacije, množični izvoz podatkov, orodja za moderiranje

**POMEMBNO**: Nikoli ne razkrivajte vašega API key v odjemalčevi kodi. API keys naj se uporabljajo samo na strežniški strani.