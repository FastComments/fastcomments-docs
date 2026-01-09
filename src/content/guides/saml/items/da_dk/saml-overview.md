SAML (Security Assertion Markup Language) er en XML-baseret åben standard for udveksling af autentificerings- og autorisationsdata mellem parter, 
især mellem en identitetsudbyder (IdP) og en tjenesteudbyder (SP).

### Hvordan SAML fungerer

SAML muliggør single sign-on (SSO) ved at lade brugere autentificere sig én gang hos deres identitetsudbyder og derefter få adgang til flere applikationer 
uden at indtaste legitimationsoplysninger igen. Når en bruger forsøger at få adgang til FastComments:

1. **Authentication Request**: FastComments viderestiller brugeren til din identitetsudbyder
2. **User Authentication**: Brugeren autentificerer sig hos din IdP (f.eks. Active Directory, Okta, Azure AD)
3. **SAML Response**: IdP sender en signeret SAML-assertion tilbage til FastComments
4. **User Access**: FastComments validerer assertionen og giver adgang til den autentificerede bruger

### Fordele ved SAML

- **Enhanced Security**: Centraliseret autentificering reducerer risici relateret til adgangskoder
- **Improved User Experience**: Brugere logger ind én gang og får adgang til flere applikationer problemfrit
- **Compliance**: Hjælper med at opfylde lovgivningsmæssige krav til adgangskontrol og revisionsspor
- **Administrative Control**: IT-administratorer opretholder centraliseret brugerstyring

### SAML 2.0 Understøttelse

FastComments implementerer SAML 2.0, den mest udbredte version af SAML-standarden. Vores implementation understøtter:

- HTTP-POST og HTTP-Redirect bindinger
- Signerede SAML-responser og assertioner
- Krypterede assertioner (valgfrit)
- Flere signatur- og digest-algoritmer
- Forskellige name identifier-formater