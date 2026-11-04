#### Sakai

Sakai ondersteunt LTI 1.3 Dynamic Registration op releases met LTI Advantage. Vanuit de Administration Workspace:

1. Log in als Sakai-beheerder en open de **Administration Workspace**.
2. Kies **External Tools** > **Install LTI 1.3 Tool**.
3. Plak de FastComments registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">verkrijg deze hier</a>) en verstuur.
4. Keur de tool goed zodra de handshake is voltooid.

De tool verschijnt vervolgens onder **External Tools** en kan door site-beheerders aan sites worden toegevoegd.

#### Schoology

Schoology Enterprise-instanties ondersteunen LTI 1.3, maar de beschikbaarheid van Dynamic Registration verschilt per implementatie. Neem contact op met je Schoology-accountmanager.

Als Dynamic Registration niet beschikbaar is op jouw Schoology-instantie, moet je de integratie handmatig configureren met behulp van deze endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nadat Schoology je een Client ID en Deployment ID heeft gegeven, neem je contact op met FastComments support om de configuratie op je tenant te registreren.

#### Andere LTI 1.3 Platforms

Elk LMS dat de IMS LTI 1.3 Advantage-specificatie volgt, zou moeten werken met dezelfde registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">verkrijg deze hier</a>). Zoek naar een instelling met de naam "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" of iets dergelijks.

Als je platform alleen handmatige LTI 1.3-configuratie ondersteunt, gebruik dan de vier endpoints die in de Schoology-sectie hierboven staan en neem contact op met support om af te ronden.