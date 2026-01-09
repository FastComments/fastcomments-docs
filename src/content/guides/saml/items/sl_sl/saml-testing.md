Preizkušanje vaše SAML konfiguracije zagotavlja, da avtentikacija deluje pravilno, preden jo uvedete med produkcijske uporabnike.

### Seznam za preverjanje pred testiranjem

Pred testiranjem SAML avtentikacije preverite:

- ✅ SAML je omogočen v FastComments
- ✅ Vsa zahtevana polja so izpolnjena (IdP URL, Certificate)
- ✅ Identitetni ponudnik je konfiguriran z informacijami FastComments SP
- ✅ V vašem IdP obstaja testni uporabniški račun
- ✅ Testnemu uporabniku so dodeljene ustrezne vloge

### Metode testiranja

#### Metoda 1: Neposredni SAML URL za prijavo

1. **Pridobite SAML URL za prijavo**:
   - Kopirajte ga s strani za konfiguracijo SAML
   - Format: `https://fastcomments.com/saml/login/{your-tenant-id}`

2. **Preizkusite avtentikacijo**:
   - Odprite SAML URL za prijavo v zasebnem/inkognito oknu brskalnika
   - Preusmerjeni bi morali biti na svoj identitetni ponudnik
   - Prijavite se s testnimi poverilnicami
   - Preverite uspešno preusmeritev nazaj na FastComments

#### Metoda 2: Dostop do skrbniške nadzorne plošče

1. **Pojdite na FastComments**:
   - Pojdite na [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
   - Poiščite možnost SAML prijave ali uporabite SAML URL za prijavo

2. **Dokončajte avtentikacijski potek**:
   - Avtenticirajte se preko svojega identitetnega ponudnika
   - Preverite dostop do ustreznih skrbniških funkcij glede na dodeljene vloge

#### Metoda 3: Testiranje integracije vtičnika

Za testiranje SAML z vtičniki za komentarje:

1. **Vdelajte vtičnik**: Uporabite FastComments vtičnik na testni strani
2. **Avtentikacija**: Kliknite prijavo in izberite možnost SAML (če je na voljo)
3. **Preverjanje**: Potrdite, da se uporabnik v vtičniku prikaže kot avtenticiran

### Kaj preveriti med testiranjem

#### Potek avtentikacije

**Uspešna preusmeritev**:
- Uporabnik je preusmerjen na prijavno stran IdP
- Prijavna stran IdP se pravilno naloži
- Ne pojavijo se napake s certifikatom ali SSL

**Avtentikacija pri IdP**:
- Uporabnik se lahko prijavi s svojimi IdP poverilnicami
- Večfaktorska avtentikacija deluje (če je konfigurirana)
- Ni napak pri avtentikaciji iz IdP

**Vrnitev v FastComments**:
- Uporabnik je po uspešni prijavi pri IdP preusmerjen nazaj v FastComments
- Ni napak pri preverjanju SAML trditve (assertion)
- Uporabnik pridobi dostop do ustreznih funkcij FastComments

#### Podatki o uporabniku

**Osnovni podatki profila**:
- E-poštni naslov je pravilno zajet
- Ime in priimek se prikažeta, če sta posredovana
- Uporabniški profil se ustvari ali posodobi

**Dodeljevanje vlog**:
- Skrbniške vloge so pravilno dodeljene
- Uporabnik ima dostop do pričakovanih skrbniških funkcij
- Dovoljenja ustrezajo dodeljenim vlogam

#### Preverjanje SAML odziva

**Preverjanje certifikata**:
- Podpis SAML odziva je uspešno preverjen
- V dnevnikih ni napak pri preverjanju certifikata
- Odziv se sprejme kot avtentičen

**Obdelava atributov**:
- Zahtevani atributi (e-pošta) so prisotni
- Neobvezni atributi so pravilno obdelani
- Atributi vlog so pravilno analizirani in uporabljeni

### Testiranje različnih scenarijev

#### Standardni potek uporabnika

1. **Nov uporabnik**:
   - Prva SAML prijava
   - Ustvarjanje računa
   - Dodelitev osnovnih dovoljenj

2. **Obstoječ uporabnik**:
   - Ponovna prijava obstoječega uporabnika
   - Posodobitve profila
   - Spremembe vlog

#### Testiranje skrbniškega dostopa

1. **Skrbniške vloge**:
   - Testni uporabniki z `fc-admin-admin` vlogo
   - Preverite dostop do skrbniške nadzorne plošče
   - Potrdite skrbniške zmogljivosti

2. **Specializirane vloge**:
   - Testirajte dostop `fc-moderator` do moderacijskih funkcij
   - Testirajte dostop `fc-analytics-admin` do analitike
   - Testirajte dostop `fc-billing-admin` do funkcij za obračunavanje

#### Scenariji napak

1. **Neveljavni certifikati**:
   - Preizkusite z poteklimi ali nepravilnimi certifikati
   - Preverite pravilno obravnavo napak

2. **Manjkajoči atributi**:
   - Preizkusite SAML odzive brez zahtevane lastnosti e-pošte
   - Preverite ustrezno obravnavo napak

3. **Težave z omrežjem**:
   - Preizkusite s težavami povezljivosti
   - Preverite obravnavo časovnih omejitev

### Odpravljanje težav pri testiranju

#### Pogoste težave z avtentikacijo

**Zanka preusmeritev**:
- Preverite, da se SP Entity ID ujema s konfiguracijo IdP
- Preverite, da je ACS URL pravilno konfiguriran
- Potrdite, da se nastavitve SAML binding ujemajo

**Napake s certifikati**:
- Prepričajte se, da certifikat vsebuje označbe BEGIN/END
- Preverite, da certifikat ni potekel
- Preverite morebitne dodatne presledke ali težave s formatiranjem

**Težave z atributi**:
- Potrdite, da se atribut e-pošte pošlje
- Preverite, da atributi vlog uporabljajo pravilno poimenovanje
- Preverite format atributov (array vs. comma-separated)

#### Orodja za odpravljanje napak

**Razvojna orodja brskalnika**:
- Spremljajte omrežne zahteve med SAML potekom
- Preverite HTTP napake ali preusmeritve
- Preverite SAML POST podatke (če so vidni)

**Orodja za testiranje IdP**:
- Večina IdP-jev nudi vmesnike za testiranje SAML
- Uporabite IdP orodja za preverjanje formata SAML odziva
- Preizkusite konfiguracijo atributov, preden jih pošljete FastComments

**Podpora FastComments**:
- Omogočite debug zapisovanje med testiranjem
- Shranite sporočila o napakah in časovne žige
- Kontaktirajte podporo s specifičnimi podatki o napaki

### Najboljše prakse testiranja

#### Nastavitev testnega okolja

1. **Posvečeni testni uporabniki**:
   - Ustvarite posebne testne račune v vašem IdP
   - Dodelite različne kombinacije vlog
   - Uporabljajte lahko prepoznavne testne e-poštne naslove

2. **Izolirano testiranje**:
   - Uporabljajte inkognito/zasebna okna brskalnika
   - Po vsakem testu počistite piškotke
   - Testirajte z različnimi uporabniškimi računi

3. **Dokumentacija**:
   - Zabeležite testne scenarije in rezultate
   - Dokumentirajte morebitne potrebne spremembe konfiguracije
   - Zapišite podrobnosti uspešne konfiguracije

#### Validacija pred produkcijo

1. **Celovito testiranje**:
   - Preizkusite vse kombinacije vlog
   - Preverite robne primere in pogoje napak
   - Potrdite, da je zmogljivost sprejemljiva

2. **Sprejemanje s strani uporabnikov**:
   - Naj končni uporabniki preizkusijo potek avtentikacije
   - Zberite povratne informacije o uporabniški izkušnji
   - Preverite, da delovni proces izpolnjuje zahteve

3. **Varnostni pregled**:
   - Potrdite, da preverjanje certifikatov deluje
   - Preverite, da so dodelitve vlog varne
   - Preizkusite izvajanje nadzora dostopa

### Uvedba v produkcijo

Po uspešnem testiranju:

1. **Postopen uvod**: Razmislite o postopnem uvajanju SAML najprej za del uporabnikov
2. **Nadzor**: Spremljajte stopnje uspešnosti avtentikacije in dnevnike napak
3. **Priprava podpore**: Pripravite podporno ekipo za vprašanja v zvezi s SAML
4. **Dokumentacija**: Zagotovite uporabniško dokumentacijo za postopek SAML prijave