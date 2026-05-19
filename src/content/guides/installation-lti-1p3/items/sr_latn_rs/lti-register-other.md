#### Sakai

Sakai podržava LTI 1.3 Dynamic Registration na izdanjima koja imaju LTI Advantage. Iz **Administration Workspace**:

1. Prijavite se kao Sakai administrator i otvorite **Administration Workspace**.
2. Izaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Nalepite FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ovde</a>) i pošaljite.
4. Odobrite alat kada se handshake završi.

Alat će se potom pojaviti pod **External Tools** i mogu ga dodati održavaoci sajtova.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost Dynamic Registration varira u zavisnosti od deploymenta. Obratite se svom Schoology account manageru.

Ako Dynamic Registration nije dostupna na vašoj Schoology instanci, moraćete da konfigurišete integraciju ručno koristeći sledeće endpoint-e:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodeli Client ID i Deployment ID, kontaktirajte FastComments podršku da registrujete konfiguraciju na vašem tenant-u.

#### Other LTI 1.3 Platforms

Bilo koji LMS koji prati IMS LTI 1.3 Advantage spec trebalo bi da radi sa istim registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ovde</a>). Potražite podešavanje označeno kao "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", ili slično.

Ako vaša platforma podržava samo ručno podešavanje LTI 1.3, koristite četiri endpoint-a navedena u odeljku Schoology iznad i kontaktirajte podršku da finalizujete postupak.