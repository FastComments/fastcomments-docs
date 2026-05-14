#### Sakai

Sakai podržava LTI 1.3 Dynamic Registration na izdanjima koja imaju LTI Advantage. Iz Administration Workspace:

1. Prijavite se kao Sakai administrator i otvorite **Administration Workspace**.
2. Odaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Zalijepite URL za registraciju FastComments i pošaljite.
4. Odobrite alat kada se handshake završi.

Alat će se tada pojaviti pod **External Tools** i mogu ga dodati održavaoci sajtova.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost Dynamic Registration varira u zavisnosti od implementacije. Provjerite sa svojim Schoology menadžerom računa.

Ako Dynamic Registration nije dostupan na vašoj Schoology instanci, moraćete ručno konfigurirati integraciju koristeći ove krajnje tačke:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodijeli Client ID i Deployment ID, kontaktirajte podršku FastComments-a da registruje konfiguraciju na vašem tenant-u.

#### Ostale LTI 1.3 platforme

Bilo koji LMS koji prati IMS LTI 1.3 Advantage specifikaciju trebao bi raditi sa istim registration URL-om. Potražite postavku označenu kao "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", ili slično.

Ako vaša platforma podržava samo ručno podešavanje LTI 1.3, koristite četiri krajnje tačke navedene u odeljku Schoology iznad i kontaktirajte podršku da finalizuje konfiguraciju.