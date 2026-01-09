Ko je SAML vklopljen v FastComments, sistem samodejno ustvari informacije ponudnika storitev (SP), ki jih morate konfigurirati v svojem ponudniku identitete.

### Dostop do informacij ponudnika storitev

Informacije SP so prikazane na vaši strani za konfiguracijo SAML po vklopu SAML overjanja. Te informacije vključujejo vse podrobnosti, ki jih vaš ponudnik identitete potrebuje za vzpostavitev SAML zaupanja.

### Končne točke ponudnika storitev

#### SP Entity ID / Dovzetnik
**Namen**: Edinstveno identificira vašo FastComments instanco kot ponudnika storitev  
**Oblika**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Uporaba**: Konfigurirajte to kot Entity ID ali Audience v vašem IdP

Ta identifikator zagotavlja, da so SAML odgovori namenjeni vašemu konkretnemu najemniku FastComments in preprečuje sprejemanje SAML odgovorov s strani drugih instanc.

#### Assertion Consumer Service (ACS) URL
**Namen**: Končna točka, kamor vaš IdP pošlje SAML odgovore po overitvi uporabnika  
**Oblika**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Uporaba**: Konfigurirajte to kot ACS URL ali Reply URL v vašem IdP

To je mesto, kamor so uporabniki preusmerjeni po uspešni overitvi pri vašem ponudniku identitete, skupaj s SAML trditvijo, ki vsebuje informacije o uporabniku.

#### SP Metadata URL
**Namen**: Nudi celotno SAML konfiguracijo v standardni XML obliki  
**Oblika**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Uporaba**: Nekateri IdP-ji lahko samodejno uvozijo konfiguracijo z uporabo tega URL-ja

URL metapodatkov vsebuje vse potrebne informacije SP v XML formatu, kar olajša samodejno konfiguracijo združljivih ponudnikov identitete.

#### SAML Login URL
**Namen**: Neposredna povezava za začetek SAML overjanja za vaš najemnik  
**Oblika**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Uporaba**: Povežite uporabnike neposredno do SAML overjanja ali preizkusite potek

Ta URL lahko uporabite za testiranje SAML overjanja ali za zagotavljanje neposredne povezave uporabnikom za prijavo preko SAML.

### Podpora SAML vezavam

FastComments podpira naslednje SAML vezave:

#### HTTP-POST vezava
- **Primarni način**: Najpogostejša vezava za SAML odgovore  
- **Varnost**: SAML odgovor se pošlje preko HTTP POST na ACS URL  
- **Uporaba**: Priporočeno za produkcijske namestitve

#### HTTP-Redirect vezava
- **Alternativni način**: SAML odgovor se pošlje preko HTTP preusmeritve  
- **Omejitve**: Omejena velikost predmeta zaradi omejitev dolžine URL-ja  
- **Uporaba**: Podprto, vendar je HTTP-POST prednostna izbira

### Politika Name ID

FastComments konfigurira naslednjo politiko Name ID v SAML zahtevah:

- **Privzeta oblika**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternativne oblike**: Persistent, Transient, Unspecified (konfigurabilno)  
- **Zahteva**: E-poštni naslov se uporablja kot primarni identifikator uporabnika

### Atributi SAML zahtev

Ko se sproži SAML overjanje, FastComments pošlje zahteve s temi značilnostmi:

#### Podpisovanje zahtev
- **Stanje**: Neobvezno (konfigurabilno)  
- **Algoritem**: Usklaja se s konfiguriranim algoritmom podpisa  
- **Certifikat**: Uporablja certifikat specifičen za najemnika, če je podpisovanje zahtev omogočeno

#### Zahtevani atributi
FastComments zahteva naslednje atribute v SAML AuthnRequests:

- **E-pošta**: Obvezno za identifikacijo uporabnika  
- **Ime**: Neobvezno za prikaz  
- **Priimek**: Neobvezno za prikaz  
- **Vloge/Skupine**: Neobvezno za nadzor dostopa in dovoljenja

### Kopiranje informacij SP

Stran za konfiguracijo SAML nudi klikabilna polja, ki samodejno kopirajo informacije SP v vaš odložišče:

1. Kliknite katerokoli polje z informacijami SP (Entity ID, ACS URL itd.)  
2. Vrednost se samodejno kopira v vaše odložišče  
3. Prilepite vrednost v konfiguracijo vašega ponudnika identitete  
4. Kratka osvetlitev označi uspešno kopiranje

To olajša natančen prenos informacij SP v vaš IdP brez tipkarskih napak.

### Informacije o certifikatu SP

#### Uporaba certifikata
- **Namen**: Šifrira komunikacijo in preverja identiteto SP  
- **Rotacija**: Certifikati so samodejno upravljani s strani FastComments  
- **Dostop**: Javne certifikate je mogoče pridobiti preko URL-ja metapodatkov

#### Podrobnosti certifikata
- **Algoritem**: RSA-2048 ali več  
- **Veljavnost**: Certifikati se samodejno obnavljajo pred potekom  
- **Distribucija**: Na voljo preko standardnih SAML metapodatkov

### Odpravljanje težav s konfiguracijo SP

Če vaš ponudnik identitete poroča o težavah z informacijami SP:

1. **Preverite URL-je**: Prepričajte se, da vsi URL-ji uporabljajo HTTPS in vključujejo pravilen ID najemnika  
2. **Preverite metapodatke**: Uporabite URL metapodatkov za preverjanje konfiguracije  
3. **Preizkusite povezljivost**: Prepričajte se, da vaš IdP lahko doseže FastComments končne točke  
4. **Preverite format**: Potrdite, da vaš IdP podpira obliko informacij SP

Pogoste težave vključujejo:
- Nepravilen ID najemnika v URL-jih  
- Težave s omrežno povezljivostjo med IdP in FastComments  
- IdP pričakuje drugačne oblike URL-jev ali dodatne možnosti konfiguracije