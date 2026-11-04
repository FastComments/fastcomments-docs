FastComments koppelt SAML-gebruikersrollen aan interne machtigingen, waardoor op rollen gebaseerde toegangscontrole mogelijk is voor uw organisatie.

### FastComments Rollensysteem

FastComments gebruikt een op rollen gebaseerd machtigingssysteem waarbij gebruikers een of meerdere rollen kunnen hebben die hun toegangsniveaus en mogelijkheden bepalen.

### Beschikbare FastComments Rollen

#### Administratieve Rollen

**fc-account-owner**
- **Machtigingen**: Volledige administratieve toegang
- **Mogelijkheden**: Alle functies, facturatiebeheer, gebruikersbeheer
- **Gebruiksscenario**: Primaire accountbeheerders en eigenaren

**fc-admin-admin**  
- **Machtigingen**: Administratieve toegang tot de meeste functies
- **Mogelijkheden**: Gebruikersbeheer, configuratie, moderatie. **Kan andere beheerders beheren.**
- **Gebruiksscenario**: Secundaire beheerders en IT-personeel

**fc-billing-admin**
- **Machtigingen**: Facturatie- en abonnementsbeheer
- **Mogelijkheden**: Betaalmethoden, facturen, abonnementswijzigingen
- **Gebruiksscenario**: Leden van het financiële team en facturatiecontacten

#### Gespecialiseerde Rollen

**fc-analytics-admin**
- **Machtigingen**: Toegang tot analyses en rapportage
- **Mogelijkheden**: Bekijk sitestatistieken, gegevens over gebruikersbetrokkenheid
- **Gebruiksscenario**: Marketingteams en data-analisten

**fc-api-admin**
- **Machtigingen**: API-toegang en -beheer
- **Mogelijkheden**: API-referenties, webhookconfiguratie
- **Gebruiksscenario**: Ontwikkelaars en technische integrators

**fc-moderator**
- **Machtigingen**: Mogelijkheden voor commentaarmoderatie
- **Mogelijkheden**: Commentaar goedkeuren/afwijzen, spam beheren
- **Gebruiksscenario**: Communitymoderators en contentmanagers

### Configuratie van Roltoewijzing

#### SAML-attribuutbronnen

FastComments accepteert rolinformatie via verschillende SAML-attribuutnamen om compatibiliteit met verschillende identity providers te waarborgen:

**Standaard attribuutnamen**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS-attributen**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Ondersteuning voor rolformaten

**Array-formaat** *(Voorkeur)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Komengescheiden formaat**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Enkelvoudig rolformaat**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Rolconfiguratie bij Identity Providers

#### Microsoft Azure AD

1. **App-rollenconfiguratie**:
   - Definieer FastComments-rollen in uw Azure AD-applicatie
   - Wijs gebruikers toe aan de juiste app-rollen
   - Configureer claims om toegewezen rollen op te nemen

2. **Attribuuttoewijzing**:
   ```
   Attribuutnaam: roles
   Bronattribuut: user.assignedroles
   ```

#### Okta

1. **Groepstoewijzing**:
   - Maak groepen aan die overeenkomen met FastComments-rolnamen
   - Wijs gebruikers toe aan de juiste groepen
   - Configureer attribuutverklaringen

2. **Attribuutverklaring**:
   ```
   Naam: roles
   Waarde: user.groups
   Filter: Begint met "fc-"
   ```

#### Google Workspace

1. **Groepstoewijzing**:
   - Maak organisatie-eenheden of groepen aan
   - Geef groepen namen met FastComments-rolprefixen
   - Configureer attribuuttoewijzing

2. **Aangepaste attributen**:
   ```
   Attribuutnaam: roles
   Waarde: Groepen of aangepast schema-attribuut
   ```

### Standaardgedrag van Gebruikers

#### Gebruikers Zonder Rollen

Wanneer een SAML-gebruiker geen rollen of onherkende rollen heeft:
- De gebruiker wordt aangemaakt als standaardcommentator
- Er wordt geen administratieve toegang verleend
- Kan eigen commentaar plaatsen en beheren
- Kan geen toegang krijgen tot functies van het beheerdersdashboard

#### Rolovererving

- Gebruikers kunnen meerdere rollen tegelijkertijd hebben
- Machtigingen zijn cumulatief (hoogste machtigingsniveau geldt)
- Rolwijzigingen in de IdP worden weerspiegeld bij de volgende login

### Beheer van SAML-gebruikers

#### Gebruikersaanmaak

Wanneer een gebruiker voor het eerst inlogt via SAML:
1. **Gebruikersaccount**: Automatisch aangemaakt met e-mail als identificatie
2. **Roltoewijzing**: Rollen toegepast op basis van SAML-attributen
3. **Profielinformatie**: Voor-/achternaam ingevuld indien opgegeven
4. **Machtigingsactivering**: Rollen worden onmiddellijk actief

#### Rolupdates

Bestaande SAML-gebruikers ontvangen rolupdates:
1. **Login-trigger**: Rolupdates vinden plaats bij elke SAML-login
2. **Onmiddellijk effect**: Nieuwe machtigingen gelden onmiddellijk
3. **Rolverwijdering**: Verwijderde rollen worden automatisch ingetrokken
4. **Audittrail**: Rolwijzigingen worden geregistreerd in auditlogboeken

### Aangepaste Roltoewijzing

#### Enterprise-aanpassing

Voor enterprise-klanten met specifieke vereisten:
- Aangepaste rolnamen kunnen worden toegewezen aan FastComments-machtigingen
- Complexe rolhiërarchieën kunnen worden geïmplementeerd
- Afdelingsspecifieke toegangscontroles kunnen worden geconfigureerd

Neem contact op met FastComments-ondersteuning voor aangepaste roltoewijzingsconfiguraties.

#### Rolvalidatie

FastComments valideert binnenkomende rollen:
- Onherkende rollen worden genegeerd (niet afgewezen)
- Misvormde rolattributen worden gelogd voor probleemoplossing
- Gebruikers behouden bestaande rollen als de SAML-assertie geen rolinformatie bevat

### Beste Praktijken

#### Rolbeheer

1. **Principe van minimale privileges**: Wijs de minimaal noodzakelijke machtigingen toe
2. **Regelmatige auditing**: Controleer gebruikersrollen en toegang periodiek  
3. **Duidelijke naamgeving**: Gebruik beschrijvende groepsnamen in uw IdP
4. **Documentatie**: Onderhoud documentatie van roltoewijzingen

#### Beveiligingsoverwegingen

1. **Rolattributen**: Zorg ervoor dat rolattributen goed beveiligd zijn in SAML-responses
2. **Attribuutvalidatie**: Controleer dat alleen geautoriseerde systemen rollen kunnen toewijzen
3. **Toegangsreviews**: Beoordeel administratieve roltoewijzingen regelmatig
4. **Monitoring**: Monitor rolwijzigingen en administratieve acties

### Problemen met Rollen Oplossen

#### Veelvoorkomende Problemen

**Rollen Niet Toegepast**:
- Controleer of SAML-attribuutnamen overeenkomen met ondersteunde formaten
- Controleer of de IdP rolinformatie verzendt
- Bevestig dat rolwaarden exact overeenkomen met FastComments-rolnamen

**Toegang Geweigerd**:
- Controleer of de gebruiker de juiste rol heeft toegewezen in de IdP
- Controleer de spelling van rollen en hoofdlettergevoeligheid
- Bevestig dat de rol correct is geformatteerd in de SAML-response

**Ontbrekende Machtigingen**:
- Bekijk roldefinities en vereiste machtigingen
- Controleer op conflicterende roltoewijzingen
- Controleer of de gebruiker heeft ingelogd na rolwijzigingen