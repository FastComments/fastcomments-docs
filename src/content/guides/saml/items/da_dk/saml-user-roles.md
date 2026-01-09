FastComments kortlægger SAML-brugerroller til interne tilladelser, hvilket muliggør rollebaseret adgangskontrol for din organisation.

### FastComments Rollesystem

FastComments bruger et rollebaseret tilladelsessystem, hvor brugere kan have en eller flere roller, som bestemmer deres adgangsniveauer og muligheder.

### Tilgængelige FastComments-roller

#### Administrative roller

**fc-account-owner**
- **Tilladelser**: Fuld administrativ adgang
- **Muligheder**: Alle funktioner, faktureringsstyring, brugerstyring
- **Brugssag**: Primære kontoadministratorer og ejere

**fc-admin-admin**  
- **Tilladelser**: Administrativ adgang til de fleste funktioner
- **Muligheder**: Brugerstyring, konfiguration, moderation. **Kan administrere andre administratorer.**
- **Brugssag**: Sekundære administratorer og IT-personale

**fc-billing-admin**
- **Tilladelser**: Fakturerings- og abonnementstyring
- **Muligheder**: Betalingsmetoder, fakturaer, abonnementsændringer
- **Brugssag**: Medarbejdere i økonomiafdelingen og faktureringskontakter

#### Specialiserede roller

**fc-analytics-admin**
- **Tilladelser**: Adgang til analyser og rapportering
- **Muligheder**: Se statistik for sider, brugerengagement-data
- **Brugssag**: Marketingteams og dataanalytikere

**fc-api-admin**
- **Tilladelser**: API-adgang og -styring
- **Muligheder**: API-legitimationsoplysninger, webhook-konfiguration
- **Brugssag**: Udviklere og tekniske integratorer

**fc-moderator**
- **Tilladelser**: Kommentarmoderationsmuligheder
- **Muligheder**: Godkende/afvise kommentarer, håndtere spam
- **Brugssag**: Fællesskabsmoderatorer og indholdsansvarlige

### Konfiguration af rolle-mapping

#### SAML-attributkilder

FastComments accepterer rolleinformation fra forskellige SAML-attributnavne for at sikre kompatibilitet med forskellige identitetsudbydere:

**Standardattributnavne**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS-attributter**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Understøttet rolleformat

**Array-format** *(Foretrukket)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Kommasepareret format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Enkelt rolle-format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Rollekonfiguration i identitetsudbyder

#### Microsoft Azure AD

1. **App-roller konfiguration**:
   - Definér FastComments-roller i din Azure AD-applikation
   - Tildel brugere de passende app-roller
   - Konfigurer claims til at inkludere tildelte roller

2. **Attributkortlægning**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Gruppetildeling**:
   - Opret grupper, der matcher FastComments-rollemavnene
   - Tildel brugere til de relevante grupper
   - Konfigurer attributudtalelser

2. **Attributudtalelse**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Gruppekortlægning**:
   - Opret organisatoriske enheder eller grupper
   - Navngiv grupper med FastComments-rollepræfikser
   - Konfigurer attributkortlægning

2. **Brugerdefinerede attributter**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Standard brugeradfærd

#### Brugere uden roller

Når en SAML-bruger ikke har roller eller har ugenkendte roller:
- Bruger oprettes som en standard kommentator
- Ingen administrativ adgang gives
- Kan poste og administrere deres egne kommentarer
- Kan ikke få adgang til administrationsdashboard-funktioner

#### Rollearv

- Brugere kan have flere roller samtidigt
- Tilladelser er kumulative (det højeste tilladelsesniveau gælder)
- Rolleændringer i IdP afspejles ved næste login

### Administrere SAML-brugere

#### Oprettelse af bruger

Når en bruger logger ind via SAML for første gang:
1. **Brugerkonto**: Oprettes automatisk med e-mail som identifikator
2. **Rolletildeling**: Roller anvendes baseret på SAML-attributter
3. **Profilinformation**: Fornavn/efternavn udfyldes, hvis det er angivet
4. **Aktivering af tilladelser**: Roller træder i kraft med det samme

#### Rolleopdateringer

Eksisterende SAML-brugere modtager rolleopdateringer:
1. **Loginudløser**: Rolleopdateringer sker under hvert SAML-login
2. **Øjeblikkelig effekt**: Nye tilladelser gælder med det samme
3. **Fjernelse af rolle**: Fjernede roller tilbagekaldes automatisk
4. **Revisionsspor**: Rolleændringer logges i revisionsloggene

### Tilpasset rollekortlægning

#### Virksomhedstilpasning

For virksomhedskunder med specifikke krav:
- Brugerdefinerede rollenavne kan kortlægges til FastComments-tilladelser
- Komplekse rollehierarkier kan implementeres
- Afdelingsspecifikke adgangskontroller kan konfigureres

Kontakt FastComments-support for konfiguration af tilpasset rollekortlægning.

#### Validering af roller

FastComments validerer indkommende roller:
- Ugenkendte roller ignoreres (ikke afvises)
- Fejlformede rolleattributter logges til fejlfinding
- Brugere bevarer eksisterende roller, hvis SAML-assertionen mangler rolleinformation

### Bedste praksis

#### Håndtering af roller

1. **Princippet om mindst privilegium**: Tildel minimalt nødvendige tilladelser
2. **Regelmæssig revision**: Gennemgå brugerroller og adgang med jævne mellemrum  
3. **Klar navngivning**: Brug beskrivende gruppenavne i din IdP
4. **Dokumentation**: Vedligehold dokumentation af rolletildelinger

#### Sikkerhedsovervejelser

1. **Rolleattributter**: Sørg for, at rolleattributter er korrekt sikret i SAML-svar
2. **Attributvalidering**: Verificer, at kun autoriserede systemer kan tildele roller
3. **Adgangsgennemgange**: Gennemgå administrative rolletildelinger regelmæssigt
4. **Overvågning**: Overvåg rolleændringer og administrative handlinger

### Fejlfinding af rolleproblemer

#### Almindelige problemer

**Roller anvendes ikke**:
- Tjek at SAML-attributnavne matcher de understøttede formater
- Verificer, at IdP sender rolleinformation
- Bekræft, at rolleværdierne matcher FastComments-rollemavnene præcist

**Adgang nægtet**:
- Verificer, at brugeren har passende rolle tildelt i IdP
- Tjek rolle-stavning og store/små bogstaver
- Bekræft, at rollen er korrekt formateret i SAML-svaret

**Manglende tilladelser**:
- Gennemgå rolledefinitioner og nødvendige tilladelser
- Tjek for modstridende rolletildelinger
- Verificer, at brugeren har logget ind efter rolleændringer

---