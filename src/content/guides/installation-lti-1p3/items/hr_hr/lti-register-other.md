#### Sakai

Sakai podržava LTI 1.3 Dynamic Registration na izdanjima s LTI Advantage. Iz Administration Workspace:

1. Prijavite se kao Sakai administrator i otvorite **Administration Workspace**.
2. Odaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Zalijepite FastComments registration URL i pošaljite.
4. Odobrite alat kada se handshake dovrši.

Alat će se potom pojaviti pod **External Tools** i njegovi održavatelji ga mogu dodati na stranice.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost Dynamic Registration varira ovisno o implementaciji. Obratite se svom Schoology account manageru.

Ako Dynamic Registration nije dostupan na vašoj Schoology instanci, morat ćete konfigurirati integraciju ručno koristeći ove endpoint-e:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodijeli Client ID i Deployment ID, kontaktirajte FastComments podršku kako bi registrirali konfiguraciju na vašem tenant-u.

#### Other LTI 1.3 Platforms

Bilo koji LMS koji slijedi IMS LTI 1.3 Advantage specifikaciju trebao bi raditi s istim registration URL-om. Potražite postavku označenu "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ili slično.

Ako vaša platforma podržava samo ručno postavljanje LTI 1.3, upotrijebite četiri endpoint-a navedena u odjeljku Schoology iznad i kontaktirajte podršku za dovršetak.