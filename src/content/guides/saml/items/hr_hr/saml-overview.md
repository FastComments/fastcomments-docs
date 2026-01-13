SAML (Security Assertion Markup Language) je otvoreni standard temeljen na XML-u za razmjenu podataka o autentifikaciji i autorizaciji između strana, posebno između davatelja identiteta (IdP) i davatelja usluge (SP).

### How SAML Works

SAML omogućuje jedinstvenu prijavu (SSO) tako da korisnicima omogućuje da se jednom autentificiraju kod svog davatelja identiteta i potom pristupe više aplikacija bez ponovnog unosa vjerodajnica. Kada korisnik pokuša pristupiti FastComments:

1. **Authentication Request**: FastComments preusmjerava korisnika na vašeg davatelja identiteta  
2. **User Authentication**: Korisnik se autentificira kod vašeg IdP-a (npr., Active Directory, Okta, Azure AD)  
3. **SAML Response**: IdP šalje potpisanu SAML tvrdnju natrag na FastComments  
4. **User Access**: FastComments provjerava tvrdnju i odobrava pristup autentificiranom korisniku

### Benefits of SAML

- **Enhanced Security**: Centralizirana autentifikacija smanjuje sigurnosne rizike povezane s lozinkama  
- **Improved User Experience**: Korisnici se jednom prijave i neometano pristupaju više aplikacija  
- **Compliance**: Pomaže u ispunjavanju regulatornih zahtjeva za kontrolu pristupa i revizijske tragove  
- **Administrative Control**: IT administratori održavaju centralizirano upravljanje korisnicima

### SAML 2.0 Support

FastComments implementira SAML 2.0, najšire prihvaćenu verziju SAML standarda. Naša implementacija podržava:

- HTTP-POST i HTTP-Redirect veze  
- Potpisani SAML odgovori i tvrdnje  
- Šifrirane tvrdnje (neobavezno)  
- Više algoritama za potpisivanje i sažimanje  
- Različiti formati identifikatora imena