FastComments ponuja tako SSO kot SAML avtentikacijo. Razumevanje razlik vam pomaga izbrati pravi pristop za vašo organizacijo.

### Simple/Secure SSO rešitve

FastComments ponuja dva različna SSO poteka za avtentikacijo v komentarni vtičnik preko vaše strani.
To se razlikuje od SAML in ne zahteva SAML. Namesto tega Preprosti SSO zahteva le posredovanje objekta v
komentarni vtičnik, medtem ko Varni SSO naredi to in dodatno hashira vsebino s pomočjo API ključa.

SAML pa avtentikira uporabnika za celoten izdelek (na podlagi njihovih dovoljenj) *kot tudi* komentarni vtičnik
(če imajo omogočene piškotke tretjih oseb za našo domeno).

### SAML avtentikacija

SAML je podjetniški protokol za avtentikacijo, ki nudi bolj robustno varnost in možnosti integracije:

- **Implementacija**: zahteva konfiguracijo ponudnika identitet (Identity Provider, IdP) in izmenjavo certifikatov
- **Varnost**: uporablja podpisane XML trditve in podpira šifriranje
- **Uporabni primer**: idealno za podjetja z obstoječo SAML infrastrukturo (Active Directory, Okta itd.)
- **Kompleksnost nastavitve**: bolj zapleteno - zahteva konfiguracijo IdP in upravljanje certifikatov
- **Podjetniške funkcije**: napredno preslikavanje vlog, centralizirano upravljanje uporabnikov, revizijske sledi

### Kdaj izbrati SAML

Razmislite o SAML avtentikaciji, če vaša organizacija:

- Že uporablja SAML-kompatibilnega ponudnika identitet (Okta, Azure AD, ADFS itd.)
- Zahteva varnost in skladnost na ravni podjetja
- Potrebuje centralizirano upravljanje uporabnikov in nadzor dostopa
- Ima več aplikacij, ki uporabljajo SAML za avtentikacijo
- Zahteva podrobne revizijske sledi in varnostno poročanje

### Kdaj izbrati Preprosti ali Varni SSO

Naše SSO rešitve, osredotočene na vtičnik, so morda zadostne, če:

- Imate lastni sistem za avtentikacijo
- Potrebujete hitro implementacijo z minimalno nastavitvijo
- Ne potrebujete integracije z enterprise ponudnikom identitet
- Želite upravljati uporabniške podatke neposredno iz vaše aplikacije
- Imate enostavnejše varnostne zahteve

Preprosti in Varni SSO se pogosto uporabljata za spletne portale, bloge itd., kjer ima uporabnik že račun *prek vaše strani ali aplikacije*
vendar ne uporablja nujno SAML.