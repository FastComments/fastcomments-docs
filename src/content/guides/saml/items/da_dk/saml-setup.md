Opsætning af SAML-autentificering i FastComments kræver både konfiguration i din administrationskonsol og opsætning i din identitetsudbyder.

### Forudsætninger

Før du konfigurerer SAML, skal du sikre dig, at du har:

- En FastComments Flex- eller Pro-plan (SAML er ikke tilgængelig på Creators-planen)
- Administrativ adgang til din FastComments-konto
- Administrativ adgang til din identitetsudbyder
- Din IdP's SAML-metadata eller certifikatinformation

### Adgang til SAML-konfiguration

1. Log ind på dit [FastComments administrationsdashboard](https://fastcomments.com/auth/my-account)
2. Gå til **API/SSO-indstillinger** i venstre sidebjælke
3. Klik på knappen **SAML-konfiguration**

Hvis du ikke kan se knappen **SAML-konfiguration**, skal du kontrollere, at:
- Din konto har den nødvendige pakke (Flex eller Pro)
- Du har administrative rettigheder
- Din bruger har rollerne API Admin eller Admin Admin

### Grundlæggende SAML-konfiguration

#### Aktivér SAML-autentificering

1. Markér afkrydsningsfeltet **Aktivér SAML-autentificering**
2. Dette aktiverer SAML for din tenant og gør konfigurationsfelterne tilgængelige

#### Obligatoriske felter

**IdP Single Sign-On URL** *(Påkrævet)*
- URL'en hvor brugere vil blive omdirigeret for autentificering
- Normalt leveret af din identitetsudbyder
- Eksempel: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509-certifikat** *(Påkrævet)*
- Det offentlige certifikat fra din identitetsudbyder
- Bruges til at verificere ægtheden af SAML-svar
- Skal inkludere det fulde certifikat med BEGIN/END-markører
- Eksempelformat:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Valgfrie felter

**IdP Entity ID / Udsteder**
- Identificerer din identitetsudbyder
- Hvis den efterlades tom, bruges som standard din FastComments-URL
- Skal matche den udsteder, der er konfigureret i din IdP

### Avanceret konfiguration

#### Sikkerhedsindstillinger

**Signaturalgoritme**
- Standard er SHA-256 (anbefalet)
- Muligheder: SHA-1, SHA-256, SHA-512
- Skal matche din IdP's konfiguration

**Digest-algoritme**
- Standard er SHA-256 (anbefalet)
- Bruges til beregning af digest i SAML-svar
- Skal matche din IdP's konfiguration

**Name ID-format**
- Standard er e-mailadresseformat
- Bestemmer, hvordan brugeridentifikatorer formateres
- Almindelige muligheder: Email Address, Persistent, Transient

#### Kryptering (valgfrit)

**Privat nøgle til dekryptering**
- Kun nødvendigt, hvis din IdP krypterer SAML-assertioner
- Indsæt din private nøgle, der bruges til dekryptering
- De fleste installationer kræver ikke kryptering af assertioner

### Gem konfiguration

1. Gennemgå alle indstillinger for korrekthed
2. Klik på **Gem SAML-konfiguration**
3. Systemet vil validere din konfiguration
4. Hvis det lykkes, vil du se en bekræftelsesmeddelelse

### Næste skridt

Efter at have gemt din FastComments SAML-konfiguration:

1. Konfigurer din identitetsudbyder ved hjælp af Service Provider-oplysningerne
2. Test autentificeringsflowet
3. Opsæt brugerroller og tilladelser efter behov

Service Provider-oplysningerne, der er nødvendige til din IdP-konfiguration, vises, når SAML er aktiveret.