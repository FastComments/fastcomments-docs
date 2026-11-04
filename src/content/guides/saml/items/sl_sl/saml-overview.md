SAML (Security Assertion Markup Language) je odprti standard, ki temelji na XML, za izmenjavo podatkov o overjanju in avtorizaciji med strankami,
predvsem med ponudnikom identitete (IdP) in ponudnikom storitev (SP).

### Kako deluje SAML

SAML omogoča enotno prijavo (SSO), saj uporabnikom omogoča, da se enkrat overijo pri svojem ponudniku identitete in nato dostopajo do več aplikacij
brez ponovnega vnašanja poverilnic. Ko uporabnik poskuša dostopati do FastComments:

1. **Zahteva za overjanje**: FastComments preusmeri uporabnika k vašemu ponudniku identitete
2. **Overjanje uporabnika**: Uporabnik se overi pri vašem IdP (npr. Active Directory, Okta, Azure AD)
3. **SAML odgovor**: IdP pošlje podpisan SAML assertion nazaj k FastComments
4. **Dostop uporabnika**: FastComments preveri assertion in podeli dostop overjenemu uporabniku

### Prednosti SAML

- **Izboljšana varnost**: Centralizirano overjanje zmanjša varnostna tveganja, povezana z gesli
- **Izboljšana uporabniška izkušnja**: Uporabniki se prijavijo enkrat in brez težav dostopajo do več aplikacij
- **Skladnost**: Pomaga izpolnjevati regulativne zahteve za nadzor dostopa in revizijske sledi
- **Administrativni nadzor**: IT skrbniki ohranjajo centralizirano upravljanje uporabnikov

### Podpora za SAML 2.0

FastComments izvaja SAML 2.0, najbolj razširjeno različico standarda SAML. Naša izvedba podpira:

- HTTP-POST in HTTP-Redirect vezave
- Podpisane SAML odgovore in assertion
- Šifrirane assertion (neobvezno)
- Več algoritmov za podpis in povzetek (digest)
- Različne formate identifikatorjev imen