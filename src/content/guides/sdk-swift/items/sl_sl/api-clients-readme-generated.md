FastComments SDK zagotavlja tri API odjemalce:

### PublicAPI - Metode, varne za odjemalca

`PublicAPI` vsebuje metode, ki so varne za klicanje iz odjemalske kode (iOS/macOS aplikacije). Te metode:
- Ne zahtevajo API ključa
- Lahko uporabljajo SSO žetone za overjanje
- So omejene po hitrosti na uporabnika/napravo
- So primerne za aplikacije, usmerjene končnemu uporabniku

**Primer uporabe**: Pridobivanje in ustvarjanje komentarjev v vaši iOS aplikaciji

### DefaultAPI - Metode na strežniški strani

`DefaultAPI` vsebuje avtenticirane metode, ki zahtevajo API ključ. Te metode:
- Zahtevajo vaš FastComments API ključ
- Morajo biti klicane le iz strežniške kode
- Omogočajo popoln dostop do vaših FastComments podatkov
- So omejene po hitrosti na najemnika

**Primer uporabe**: Administrativna opravila, množični izvoz podatkov, upravljanje uporabnikov

### ModerationAPI - Metode nadzorne plošče moderatorja

`ModerationAPI` ponuja obsežen nabor hitrih in živih moderacijskih API-jev. Vsaka metoda `ModerationAPI` sprejme parameter `sso` in se lahko overi prek SSO ali FastComments.com piškotka seje.

**Primer uporabe**: Gradnja moderacijske izkušnje za moderatorje vaše skupnosti

**POMEMBNO**: Nikoli ne razkrivajte svojega API ključa v odjemalski kodi. API ključe je treba uporabljati le na strežniku.