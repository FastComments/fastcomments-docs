#### Sakai

Sakai podržava LTI 1.3 dinamičku registraciju na izdanjima koja imaju LTI Advantage. Iz Administrativnog radnog prostora:

1. Prijavite se kao Sakai administrator i otvorite **Administrativni radni prostor**.
2. Odaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Zalijepite FastComments URL za registraciju i pošaljite.
4. Odobrite alat kada završi postupak rukovanja (handshake).

Alat će se zatim pojaviti pod **External Tools** i mogu ga dodati održavaoci sajtova.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost dinamičke registracije varira u zavisnosti od implementacije. Provjerite sa svojim Schoology menadžerom računa.

Ako dinamička registracija nije dostupna na vašoj Schoology instanci, moraćete konfigurirati integraciju ručno koristeći ove krajnje tačke:

- **OIDC URL za prijavu**: `https://fastcomments.com/lti/v1p3/login`
- **URL ciljnog linka**: `https://fastcomments.com/lti/v1p3/launch`
- **URL javnog skupa ključeva (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **URL-ovi za preusmjeravanje**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodijeli Client ID i Deployment ID, kontaktirajte FastComments podršku da registruje konfiguraciju na vašem tenant-u.

#### Ostale LTI 1.3 platforme

Bilo koji LMS koji prati IMS LTI 1.3 Advantage specifikaciju trebao bi raditi sa istim URL-om za registraciju. Potražite postavku označenu kao "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ili slično.

Ako vaša platforma podržava samo ručno postavljanje LTI 1.3, koristite četiri krajnje tačke navedene u odjeljku Schoology iznad i kontaktirajte podršku da finalizuje konfiguraciju.