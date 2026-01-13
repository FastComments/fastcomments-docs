---
FastComments mapira SAML korisničke uloge na interne dozvole, omogućavajući kontrolu pristupa zasnovanu na ulogama za vašu organizaciju.

### FastComments sistem uloga

FastComments koristi sistem dozvola zasnovan na ulogama gde korisnici mogu imati jednu ili više uloga koje određuju njihov nivo pristupa i mogućnosti.

### Dostupne FastComments uloge

#### Administrativne uloge

**fc-account-owner**
- **Dozvole**: Kompletan administratorski pristup
- **Mogućnosti**: Sve funkcije, upravljanje naplatom, upravljanje korisnicima
- **Upotreba**: Glavni administratori i vlasnici naloga

**fc-admin-admin**  
- **Dozvole**: Administrativni pristup većini funkcionalnosti
- **Mogućnosti**: Upravljanje korisnicima, konfiguracija, moderacija. **Može upravljati drugim administratorima.**
- **Upotreba**: Sekundarni administratori i IT osoblje

**fc-billing-admin**
- **Dozvole**: Upravljanje naplatom i pretplatama
- **Mogućnosti**: Metode plaćanja, fakture, promene pretplate
- **Upotreba**: Članovi finansijskog tima i kontakti za fakturisanje

#### Specijalizovane uloge

**fc-analytics-admin**
- **Dozvole**: Pristup analitici i izveštavanju
- **Mogućnosti**: Pregled statistike sajta, podataka o angažovanju korisnika
- **Upotreba**: Marketing timovi i analitičari podataka

**fc-api-admin**
- **Dozvole**: Pristup i upravljanje API-jem
- **Mogućnosti**: API kredencijali, konfiguracija webhook-ova
- **Upotreba**: Programeri i tehnički integratori

**fc-moderator**
- **Dozvole**: Mogućnosti moderacije komentara
- **Mogućnosti**: Odobravanje/odbijanje komentara, upravljanje spamom
- **Upotreba**: Moderatori zajednice i menadžeri sadržaja

### Konfiguracija mapiranja uloga

#### Izvori SAML atributa

FastComments prihvata informacije o ulogama iz različitih imena SAML atributa kako bi osigurao kompatibilnost sa različitim provajderima identiteta:

**Standardna imena atributa**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS atributi**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Podržani formati uloga

**Format niza** *(Preporučeno)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format odvojen zarezom**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format jedne uloge**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Konfiguracija uloga provajdera identiteta

#### Microsoft Azure AD

1. **Konfiguracija App uloga**:
   - Definišite FastComments uloge u vašoj Azure AD aplikaciji
   - Dodelite korisnike odgovarajućim app ulogama
   - Konfigurišite claims da uključuju dodeljene uloge

2. **Mapiranje atributa**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Dodela grupa**:
   - Kreirajte grupe koje odgovaraju imenima FastComments uloga
   - Dodelite korisnike odgovarajućim grupama
   - Konfigurišite izjave atributa

2. **Izjava atributa**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mapiranje grupa**:
   - Kreirajte organizacione jedinice ili grupe
   - Imenovati grupe sa prefiksima FastComments uloga
   - Konfigurišite mapiranje atributa

2. **Prilagođeni atributi**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Podrazumevano ponašanje korisnika

#### Korisnici bez uloga

Kada SAML korisnik nema uloge ili ima neprepoznate uloge:
- Korisnik se kreira kao standardni komentator
- Ne dodeljuju mu se administratorski pristupi
- Može da objavljuje i upravlja sopstvenim komentarima
- Ne može da pristupi administratorskom panelu

#### Nasleđivanje uloga

- Korisnici mogu istovremeno imati više uloga
- Dozvole su kumulativne (primenjuje se najviši nivo dozvola)
- Promene uloga u IdP-u se reflektuju prilikom sledeće prijave

### Upravljanje SAML korisnicima

#### Kreiranje korisnika

Kada se korisnik prvi put prijavi putem SAML-a:
1. **Korisnički nalog**: Automatski se kreira sa e-poštom kao identifikatorom
2. **Dodela uloga**: Uloge se primenjuju na osnovu SAML atributa
3. **Informacije profila**: Ime/prezime se popunjavaju ako su dostupni
4. **Aktivacija dozvola**: Uloge postaju aktivne odmah

#### Ažuriranja uloga

Postojeći SAML korisnici dobijaju ažuriranja uloga:
1. **Okidač prijave**: Ažuriranja uloga se dešavaju pri svakoj SAML prijavi
2. **Momentalni efekat**: Nove dozvole se primenjuju odmah
3. **Uklanjanje uloga**: Uklonjene uloge se automatski opozivaju
4. **Zapisnik revizije**: Promene uloga se beleže u revizijskim logovima

### Prilagođeno mapiranje uloga

#### Korporativne prilagodbe

Za korporativne korisnike sa specifičnim zahtevima:
- Prilagođena imena uloga mogu se mapirati na FastComments dozvole
- Mogu se implementirati složene hijerarhije uloga
- Mogu se konfigurisati kontrole pristupa specifične za odeljenja

Kontaktirajte FastComments podršku za konfiguracije prilagođenog mapiranja uloga.

#### Validacija uloga

FastComments validira dolazne uloge:
- Neprepoznate uloge se ignorišu (ne odbacuju se)
- Neispravni atributi uloga se beleže za rešavanje problema
- Korisnici zadržavaju postojeće uloge ako SAML izjava nema informacije o ulogama

### Najbolje prakse

#### Upravljanje ulogama

1. **Princip najmanjih privilegija**: Dodeljujte minimalno neophodne dozvole
2. **Redovna revizija**: Periodično pregledajte uloge korisnika i pristupe  
3. **Jasno imenovanje**: Koristite opisna imena grupa u vašem IdP-u
4. **Dokumentacija**: Održavajte dokumentaciju dodele uloga

#### Bezbednosne mere

1. **Atributi uloga**: Osigurajte da su atributi uloga pravilno zaštićeni u SAML odgovorima
2. **Validacija atributa**: Verifikujte da samo autorizovani sistemi mogu dodeljivati uloge
3. **Pregled pristupa**: Redovno pregledajte dodelu administratorskih uloga
4. **Monitoring**: Pratite promene uloga i administratorske aktivnosti

### Otklanjanje problema sa ulogama

#### Uobičajeni problemi

**Roles Not Applied**:
- Proverite da li imena SAML atributa odgovaraju podržanim formatima
- Verifikujte da IdP šalje informacije o ulogama
- Potvrdite da vrednosti uloga tačno odgovaraju imenima FastComments uloga

**Access Denied**:
- Proverite da li korisniku u IdP-u dodeljena odgovarajuća uloga
- Proverite pravopis i osetljivost na veličinu slova uloga
- Potvrdite da je uloga pravilno formatirana u SAML odgovoru

**Missing Permissions**:
- Pregledajte definicije uloga i potrebne dozvole
- Proverite da li postoji konflikt u dodelama uloga
- Verifikujte da se korisnik prijavio nakon promena uloga

---