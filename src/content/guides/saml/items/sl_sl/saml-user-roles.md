FastComments preslika SAML uporabniške vloge na notranje pravice, kar omogoča nadzor dostopa na podlagi vlog za vašo organizacijo.

### Sistem vlog FastComments

FastComments uporablja sistem dovoljenj na podlagi vlog, kjer lahko imajo uporabniki eno ali več vlog, ki določajo njihove ravni dostopa in funkcionalnosti.

### Razpoložljive vloge FastComments

#### Administrativne vloge

**fc-account-owner**
- **Pooblastila**: Popoln administrativni dostop
- **Funkcionalnosti**: Vse funkcije, upravljanje obračunavanja, upravljanje uporabnikov
- **Uporabni primer**: Glavni skrbniki in lastniki računa

**fc-admin-admin**  
- **Pooblastila**: Administrativni dostop do večine funkcij
- **Funkcionalnosti**: Upravljanje uporabnikov, konfiguracija, moderacija. **Lahko upravlja tudi druge administratorje.**
- **Uporabni primer**: Sekundarni administratorji in IT osebje

**fc-billing-admin**
- **Pooblastila**: Upravljanje obračunavanja in naročnin
- **Funkcionalnosti**: Načini plačila, računi, spremembe naročnin
- **Uporabni primer**: Člani finančne ekipe in kontakti za obračunavanje

#### Specializirane vloge

**fc-analytics-admin**
- **Pooblastila**: Dostop do analitike in poročanja
- **Funkcionalnosti**: Ogled statistike spletnega mesta, podatki o vključenosti uporabnikov
- **Uporabni primer**: Tržne ekipe in analitiki podatkov

**fc-api-admin**
- **Pooblastila**: Dostop do API in upravljanje
- **Funkcionalnosti**: API poverilnice, konfiguracija webhookov
- **Uporabni primer**: Razvijalci in tehnični integratorji

**fc-moderator**
- **Pooblastila**: Zmožnosti moderiranja komentarjev
- **Funkcionalnosti**: Odobravanje/zavračanje komentarjev, upravljanje neželene pošte
- **Uporabni primer**: Moderatorji skupnosti in upravljavci vsebin

### Konfiguracija preslikave vlog

#### Viri atributov SAML

FastComments sprejema podatke o vlogah iz različnih imen atributov SAML, da zagotovi združljivost z različnimi ponudniki identitete:

**Standardna imena atributov**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS atributi**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Podprti formati vlog

**Format polja** *(priporočeno)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format ločen z vejico**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format ene vloge**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Konfiguracija vlog ponudnika identitete

#### Microsoft Azure AD

1. **Konfiguracija vlog aplikacije**:
   - Določite vloge FastComments v vaši aplikaciji Azure AD
   - Dodelite uporabnike ustreznim vlogam aplikacije
   - Konfigurirajte trditve (claims), da vključujejo dodeljene vloge

2. **Preslikava atributov**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Dodeljevanje skupin**:
   - Ustvarite skupine, ki se ujemajo z imeni vlog FastComments
   - Dodelite uporabnike ustreznim skupinam
   - Konfigurirajte izjave o atributih

2. **Izjava o atributih**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Preslikava skupin**:
   - Ustvarite organizacijske enote ali skupine
   - Poimenujte skupine z FastComments predponami vlog
   - Konfigurirajte preslikavo atributov

2. **Prilagojeni atributi**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Privzeto vedenje uporabnikov

#### Uporabniki brez vlog

Ko SAML uporabnik nima vlog ali ima neprepoznane vloge:
- Uporabnik je ustvarjen kot običajen komentator
- Ne dodeli se noben administrativni dostop
- Lahko objavlja in upravlja lastne komentarje
- Ne more dostopati do funkcij upravnega vmesnika

#### Dedovanje vlog

- Uporabniki lahko imajo več vlog hkrati
- Pooblastila se seštevajo (velja najvišja raven pooblastil)
- Spremembe vlog v IdP se odrazijo pri naslednji prijavi

### Upravljanje SAML uporabnikov

#### Ustvarjanje uporabnika

Ko se uporabnik ob prvi prijavi prek SAML:
1. **Uporabniški račun**: Samodejno ustvarjen z e-pošto kot identifikator
2. **Dodelitev vloge**: Vloge se uporabijo na podlagi atributov SAML
3. **Podatki profila**: Ime in priimek izpolnjena, če sta podana
4. **Aktivacija pooblastil**: Vloge začnejo veljati takoj

#### Posodobitve vlog

Obstoječi SAML uporabniki prejmejo posodobitve vlog:
1. **Sprožilec ob prijavi**: Posodobitve vlog se izvajajo ob vsaki SAML prijavi
2. **Takojšen učinek**: Nova pooblastila se uporabijo takoj
3. **Odstranitev vlog**: Odstranjene vloge so samodejno odvzete
4. **Sledenje sprememb**: Spremembe vlog so zabeležene v revizijskih zapisih

### Prilagojena preslikava vlog

#### Prilagoditve za podjetja

Za podjetja s posebnimi zahtevami:
- Imena vlog po meri je mogoče preslikati na pooblastila FastComments
- Lahko se uvedejo kompleksne hierarhije vlog
- Možno je konfigurirati dostopne kontrole za posamezne oddelke

Kontaktirajte podporo FastComments za konfiguracije prilagojene preslikave vlog.

#### Preverjanje vlog

FastComments preverja dohodne vloge:
- Neprepoznane vloge se prezrejo (ne zavrnejo)
- Nepravilno oblikovani atributi vlog so zabeleženi za odpravljanje težav
- Uporabniki ohranijo obstoječe vloge, če SAML trditev ne vsebuje informacij o vlogah

### Najboljše prakse

#### Upravljanje vlog

1. **Načelo najmanjših pooblastil**: Dodelite minimalna potrebna pooblastila
2. **Redno revidiranje**: Periodično pregledujte uporabniške vloge in dostop  
3. **Jasno poimenovanje**: V IdP uporabljajte opisna imena skupin
4. **Dokumentacija**: Vzdržujte dokumentacijo dodelitev vlog

#### Varnostni vidiki

1. **Atributi vlog**: Poskrbite, da so atributi vlog v SAML odzivih ustrezno zaščiteni
2. **Preverjanje atributov**: Preverite, da lahko vloge dodeljujejo samo pooblaščeni sistemi
3. **Pregledi dostopa**: Redno pregledujte dodelitve administrativnih vlog
4. **Nadzor**: Spremljajte spremembe vlog in administrativna dejanja

### Odpravljanje težav z vlogami

#### Pogoste težave

**Roles Not Applied**:
- Preverite, ali se imena atributov SAML ujemajo s podprtimi formati
- Preverite, ali IdP pošilja informacije o vlogah
- Potrdite, da se vrednosti vlog natančno ujemajo z imeni vlog FastComments

**Access Denied**:
- Preverite, ali ima uporabnik v IdP dodeljeno ustrezno vlogo
- Preverite črkovanje vlog in občutljivost na velike/male črke
- Potrdite, da je vloga pravilno oblikovana v SAML odzivu

**Missing Permissions**:
- Preglejte definicije vlog in zahtevana pooblastila
- Preverite morebitne sporne dodelitve vlog
- Preverite, ali se je uporabnik prijavil po spremembah vlog

---