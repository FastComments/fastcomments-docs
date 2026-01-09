Nastavitev SAML overjanja v FastComments zahteva tako konfiguracijo v vašem upravljalskem vmesniku kot nastavitve pri vašem ponudniku identitete.

### Predpogoji

Pred konfiguracijo SAML zagotovite, da imate:

- Načrt FastComments Flex ali Pro (SAML ni na voljo v načrtu Creators)
- Administrativni dostop do vašega FastComments računa
- Administrativni dostop do vašega ponudnika identitete
- SAML metapodatke ali informacije o certifikatu vašega IdP

### Dostop do konfiguracije SAML

1. Prijavite se v vaš [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Navigirajte do **API/SSO Settings** v levem stranskem meniju
3. Kliknite gumb **SAML Config**

Če ne vidite gumba **SAML Config**, preverite, da:
- Vaš račun ima zahtevan paket (Flex ali Pro)
- Imate administratorske pravice
- Vaš uporabnik ima vloge API Admin ali Admin Admin

### Osnovna konfiguracija SAML

#### Omogočanje SAML overjanja

1. Označite potrditveno polje **Enable SAML Authentication**
2. To aktivira SAML za vašega najemnika in omogoči polja konfiguracije

#### Obvezna polja

**IdP Single Sign-On URL** *(Obvezno)*
- URL, na katerega bodo uporabniki preusmerjeni za overjanje
- Običajno ga zagotovi vaš ponudnik identitete
- Primer: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Obvezno)*
- Javni certifikat vašega ponudnika identitete
- Uporablja se za preverjanje pristnosti SAML odgovorov
- Mora vsebovati celoten certifikat z oznakami BEGIN/END
- Primer formata:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Neobvezna polja

**IdP Entity ID / Issuer**
- Identificira vašega ponudnika identitete
- Če je prazno, privzeto uporabi vaš FastComments URL
- Mora se ujemati z izdajateljem, nastavljenim v vašem IdP

### Napredna konfiguracija

#### Varnostne nastavitve

**Signature Algorithm**
- Privzeto SHA-256 (priporočeno)
- Možnosti: SHA-1, SHA-256, SHA-512
- Mora ustrezati konfiguraciji vašega IdP

**Digest Algorithm**
- Privzeto SHA-256 (priporočeno)
- Uporablja se za izračun zgoščevanja v SAML odgovorih
- Mora ustrezati konfiguraciji vašega IdP

**Name ID Format**
- Privzeto: oblika e-poštnega naslova
- Določa, kako so oblikovani identifikatorji uporabnikov
- Pogoste možnosti: Email Address, Persistent, Transient

#### Šifriranje (neobvezno)

**Private Key for Decryption**
- Potreben samo, če vaš IdP šifrira SAML trditve
- Prilepite vaš zasebni ključ, uporabljen za dešifriranje
- Večina namestitev ne zahteva šifriranja trditev

### Shranjevanje konfiguracije

1. Preglejte vse nastavitve za točnost
2. Kliknite **Save SAML Configuration**
3. Sistem bo preveril vašo konfiguracijo
4. Če je uspešno, boste videli potrdilno sporočilo

### Naslednji koraki

Po shranitvi vaše FastComments SAML konfiguracije:

1. Konfigurirajte vašega ponudnika identitete z uporabo informacij o ponudniku storitev
2. Preizkusite postopek overjanja
3. Nastavite uporabniške vloge in dovoljenja po potrebi

Informacije o ponudniku storitev, potrebne za konfiguracijo vašega IdP, bodo prikazane, ko bo SAML omogočen.