#### Sakai

Sakai podržava LTI 1.3 dinamičku registraciju u izdanjima s LTI Advantage. Iz Administracijskog radnog prostora:

1. Prijavite se kao Sakai administrator i otvorite **Administracijski radni prostor**.
2. Odaberite **External Tools** > **Install LTI 1.3 Tool**.
3. Zalijepite FastComments registracijski URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>) i pošaljite.
4. Odobrite alat kada se uspostavi vezanje (handshake).

Alat se zatim pojavljuje pod **External Tools** i njegovi održavatelji ga mogu dodati na stranice.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost dinamičke registracije varira ovisno o implementaciji. Provjerite kod svog Schoology voditelja računa.

Ako dinamička registracija nije dostupna na vašoj Schoology instanci, morat ćete konfigurirati integraciju ručno koristeći ove krajnje točke:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodijeli Client ID i Deployment ID, kontaktirajte FastComments podršku da registrira konfiguraciju na vašem tenant-u.

#### Ostale LTI 1.3 platforme

Bilo koji LMS koji slijedi IMS LTI 1.3 Advantage specifikaciju trebao bi raditi s istim registracijskim URL-om (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>). Potražite postavku označenu kao "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ili slično.

Ako vaša platforma podržava samo ručno postavljanje LTI 1.3, koristite četiri gore navedene krajnje točke iz odjeljka Schoology i kontaktirajte podršku da dovršite postupak.