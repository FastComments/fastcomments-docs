#### Sakai

Sakai ondersteunt LTI 1.3 Dynamische registratie in releases met LTI Advantage. Vanuit de Beheerwerkruimte:

1. Meld u aan als Sakai-beheerder en open de **Beheerwerkruimte**.
2. Kies **Externe hulpmiddelen** > **Installeer LTI 1.3-hulpmiddel**.
3. Plak de FastComments-registratie-URL en verstuur.
4. Keur het hulpmiddel goed wanneer de handshake is voltooid.

Het hulpmiddel verschijnt vervolgens onder **Externe hulpmiddelen** en kan door sitebeheerders aan sites worden toegevoegd.

#### Schoology

Schoology Enterprise-instanties ondersteunen LTI 1.3, maar de beschikbaarheid van Dynamische registratie varieert per implementatie. Neem contact op met uw Schoology-accountmanager.

Als Dynamische registratie niet beschikbaar is op uw Schoology-instantie, moet u de integratie handmatig configureren met behulp van deze endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nadat Schoology u een Client ID en Deployment ID heeft gegeven, neem contact op met FastComments-ondersteuning om de configuratie op uw tenant te registreren.

#### Andere LTI 1.3-platforms

Elke LMS die de IMS LTI 1.3 Advantage-specificatie volgt zou moeten werken met dezelfde registratie-URL. Zoek naar een instelling met het label "Dynamische registratie", "Tool-registratie-URL", "Tool-initiatie-registratie-eindpunt", of iets dergelijks.

Als uw platform alleen handmatige LTI 1.3-configuratie ondersteunt, gebruik dan de vier endpoints die hierboven in de Schoology-sectie zijn vermeld en neem contact op met ondersteuning om de configuratie af te ronden.