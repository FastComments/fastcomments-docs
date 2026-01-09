SAML (Security Assertion Markup Language) je na XML temelječ odprt standard za izmenjavo podatkov o avtentikaciji in avtorizaciji med strankami, 
predvsem med ponudnikom identitete (IdP) in ponudnikom storitev (SP).

### How SAML Works

SAML omogoča enkratno prijavo (SSO) tako, da uporabnikom dovoli, da se enkrat avtenticirajo pri svojem ponudniku identitete in nato dostopajo do več aplikacij 
brez ponovnega vnašanja poverilnic. Ko uporabnik poskuša dostopati do FastComments:

1. **Authentication Request**: FastComments preusmeri uporabnika na vašega ponudnika identitete
2. **User Authentication**: Uporabnik se avtenticira pri vašem IdP-ju (npr. Active Directory, Okta, Azure AD)
3. **SAML Response**: IdP pošlje podpisano SAML trditev nazaj FastComments
4. **User Access**: FastComments preveri trditev in podeli dostop avtenticiranemu uporabniku

### Benefits of SAML

- **Enhanced Security**: Centralizirana avtentikacija zmanjšuje varnostna tveganja, povezana z gesli
- **Improved User Experience**: Uporabniki se prijavijo enkrat in nato brez težav dostopajo do več aplikacij
- **Compliance**: Pomaga izpolnjevati regulativne zahteve za nadzor dostopa in revizijske sledi
- **Administrative Control**: IT skrbniki ohranjajo centralizirano upravljanje uporabnikov

### SAML 2.0 Support

FastComments implementira SAML 2.0, najbolj razširjeno različico standarda SAML. Naša implementacija podpira:

- HTTP-POST in HTTP-Redirect vezave
- Podpisani SAML odgovori in trditve
- Šifrirane trditve (neobvezno)
- Več algoritmov za podpis in zgoščevanje
- Različne formate identifikatorjev imen