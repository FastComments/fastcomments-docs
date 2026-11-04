#### Sakai

Sakai podržava LTI 1.3 Dynamic Registration na verzijama sa LTI Advantage. Iz Administration Workspace-a:

1. Prijavite se kao Sakai admin i otvorite **Administration Workspace**.
2. Izaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Nalepite FastComments registracioni URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovde</a>) i pošaljite.
4. Odobrite alat kada se handshake završi.

Alat se zatim pojavljuje pod **External Tools** i može se dodati na sajtove od strane njihovih održavalaca.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost Dynamic Registration varira zavisno od deployment-a. Proverite sa svojim Schoology account menadžerom.

Ako Dynamic Registration nije dostupan na vašoj Schoology instanci, potrebno je da konfigurišete integraciju ručno koristeći sledeće endpoint-ove:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što Schoology vam da Client ID i Deployment ID, kontaktirajte FastComments podršku da registruje konfiguraciju na vašem tenant-u.

#### Ostale LTI 1.3 Platforme

Svaki LMS koji prati IMS LTI 1.3 Advantage specifikaciju trebalo bi da radi sa istim registracionim URL-om (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovde</a>). Potražite podešavanje označeno kao "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ili slično.

Ako vaša platforma podržava samo ručno LTI 1.3 podešavanje, koristite četiri endpoint-a navedena u Schoology sekciji iznad i kontaktirajte podršku da završite proces.