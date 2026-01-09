FastComments preslikava SAML korisničke uloge u interne dozvole, omogućujući kontrolu pristupa temeljenu na ulogama za vašu organizaciju.

### Sustav uloga FastComments

FastComments koristi sustav dozvola temeljen na ulogama gdje korisnici mogu imati jednu ili više uloga koje određuju njihove razine pristupa i mogućnosti.

### Dostupne uloge u FastComments

#### Administrativne uloge

**fc-account-owner**
- **Permissions**: Potpun administrativni pristup
- **Capabilities**: Sve značajke, upravljanje naplatom, upravljanje korisnicima
- **Use Case**: Glavni administratori i vlasnici računa

**fc-admin-admin**  
- **Permissions**: Administrativni pristup većini značajki
- **Capabilities**: Upravljanje korisnicima, konfiguracija, moderacija. **Može upravljati drugim administratorima.**
- **Use Case**: Sekundarni administratori i IT osoblje

**fc-billing-admin**
- **Permissions**: Upravljanje naplatom i pretplatama
- **Capabilities**: Načini plaćanja, računi, promjene pretplate
- **Use Case**: Članovi financijskog tima i kontakti za naplatu

#### Specijalizirane uloge

**fc-analytics-admin**
- **Permissions**: Pristup analitici i izvještavanju
- **Capabilities**: Pregled statistika stranice, podataka o angažmanu korisnika
- **Use Case**: Marketinški timovi i analitičari podataka

**fc-api-admin**
- **Permissions**: Pristup API-ju i upravljanje
- **Capabilities**: API vjerodajnice, konfiguracija webhookova
- **Use Case**: Programeri i tehnički integratori

**fc-moderator**
- **Permissions**: Mogućnosti moderiranja komentara
- **Capabilities**: Odobravanje/odbijanje komentara, upravljanje spamom
- **Use Case**: Moderatori zajednice i urednici sadržaja

### Konfiguracija preslikavanja uloga

#### Izvori SAML atributa

FastComments prihvaća informacije o ulogama iz raznih SAML naziva atributa kako bi osigurao kompatibilnost s različitim davateljima identiteta:

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

**Array Format** *(Preporučeno)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Konfiguracija uloga u davatelju identiteta

#### Microsoft Azure AD

1. **Konfiguracija uloga aplikacije**:
   - Definirajte FastComments uloge u vašoj Azure AD aplikaciji
   - Dodijelite korisnike odgovarajućim ulogama aplikacije
   - Konfigurirajte tvrdnje (claims) da uključuju dodijeljene uloge

2. **Preslikavanje atributa**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Dodjela grupa**:
   - Kreirajte grupe koje odgovaraju imenima FastComments uloga
   - Dodijelite korisnike odgovarajućim grupama
   - Konfigurirajte izjave o atributima

2. **Izjava o atributu**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mapiranje grupa**:
   - Kreirajte organizacijske jedinice ili grupe
   - Imenujte grupe s prefiksima FastComments uloga
   - Konfigurirajte preslikavanje atributa

2. **Prilagođeni atributi**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Zadano ponašanje korisnika

#### Korisnici bez uloga

Kada SAML korisnik nema uloge ili ima nepriznate uloge:
- Korisnik se stvara kao standardni komentator
- Nije dodijeljen administrativni pristup
- Može objavljivati i upravljati vlastitim komentarima
- Ne može pristupiti značajkama administratorske nadzorne ploče

#### Nasljeđivanje uloga

- Korisnici mogu imati više uloga istovremeno
- Dozvole se zbrajaju (primjenjuje se najviša razina dozvola)
- Promjene uloga u davatelju identiteta (IdP) odražavaju se pri sljedećem prijavljivanju

### Upravljanje SAML korisnicima

#### Kreiranje korisnika

Kada se korisnik prvi put prijavi putem SAML-a:
1. **User Account**: Automatski se stvara s e-poštom kao identifikatorom
2. **Role Assignment**: Uloge se primjenjuju na temelju SAML atributa
3. **Profile Information**: Ime/prezime popunjavaju se ako su navedeni
4. **Permission Activation**: Uloge postaju aktivne odmah

#### Ažuriranja uloga

Postojeći SAML korisnici primaju ažuriranja uloga:
1. **Login Trigger**: Ažuriranja uloga se događaju pri svakoj SAML prijavi
2. **Immediate Effect**: Nove dozvole se primjenjuju odmah
3. **Role Removal**: Uklonjene uloge se automatski opozivaju
4. **Audit Trail**: Promjene uloga se bilježe u dnevnicima revizije

### Prilagođeno preslikavanje uloga

#### Prilagodba za poduzeća

Za korporativne korisnike s posebnim zahtjevima:
- Prilagođena imena uloga mogu se preslikati na FastComments dozvole
- Mogu se implementirati složene hijerarhije uloga
- Mogu se konfigurirati kontrole pristupa specifične za odjele

Kontaktirajte FastComments podršku za konfiguracije prilagođenog preslikavanja uloga.

#### Validacija uloga

FastComments validira dolazne uloge:
- Neprepoznate uloge se ignoriraju (ne odbacuju)
- Neispravno oblikovani atributi uloga se bilježe za otklanjanje poteškoća
- Korisnici zadržavaju postojeće uloge ako SAML izjava ne sadrži informacije o ulogama

### Najbolje prakse

#### Upravljanje ulogama

1. **Principle of Least Privilege**: Dodijelite minimalne potrebne dozvole
2. **Regular Auditing**: Periodično pregledavajte uloge korisnika i pristup  
3. **Clear Naming**: Koristite opisna imena grupa u vašem IdP-u
4. **Documentation**: Održavajte dokumentaciju dodjela uloga

#### Sigurnosne napomene

1. **Role Attributes**: Osigurajte da su atributi uloga pravilno zaštićeni u SAML odgovorima
2. **Attribute Validation**: Provjerite da samo ovlašteni sustavi mogu dodjeljivati uloge
3. **Access Reviews**: Redovito pregledavajte dodjele administratorskih uloga
4. **Monitoring**: Pratite promjene uloga i administrativne radnje

### Otklanjanje poteškoća s ulogama

#### Uobičajeni problemi

**Uloge nisu primijenjene**:
- Provjerite da imena SAML atributa odgovaraju podržanim formatima
- Provjerite da IdP šalje informacije o ulogama
- Potvrdite da vrijednosti uloga točno odgovaraju imenima FastComments uloga

**Pristup odbijen**:
- Provjerite da korisnik ima odgovarajuću ulogu dodijeljenu u IdP-u
- Provjerite pravopis uloge i osjetljivost na velika/mala slova
- Potvrdite da je uloga pravilno formatirana u SAML odgovoru

**Nedostajuće dozvole**:
- Pregledajte definicije uloga i potrebne dozvole
- Provjerite ima li sukobljenih dodjela uloga
- Provjerite je li se korisnik prijavio nakon promjena uloga