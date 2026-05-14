#### Sakai

Sakai ondersteunt LTI 1.3 Dynamic Registration op releases met LTI Advantage. Vanuit de Administratiewerkruimte:

1. Meld u aan als Sakai-beheerder en open de **Administratiewerkruimte**.
2. Kies **Externe Tools** > **Installeer LTI 1.3-tool**.
3. Plak de FastComments-registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal deze hier</a>) en verstuur.
4. Keur de tool goed wanneer de handshake is voltooid.

De tool verschijnt vervolgens onder **Externe Tools** en kan door sitebeheerders aan sites worden toegevoegd.

#### Schoology

Schoology Enterprise-instanties ondersteunen LTI 1.3, maar de beschikbaarheid van Dynamic Registration varieert per implementatie. Neem contact op met uw Schoology-accountmanager.

Als Dynamic Registration niet beschikbaar is op uw Schoology-instantie, moet u de integratie handmatig configureren met behulp van deze endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nadat Schoology u een Client ID en Deployment ID heeft gegeven, neem contact op met FastComments-support om de configuratie voor uw tenant te registreren.

#### Other LTI 1.3 Platforms

Elke LMS die de IMS LTI 1.3 Advantage-specificatie volgt, zou moeten werken met dezelfde registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal deze hier</a>). Zoek naar een instelling met het label "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", of iets soortgelijks.

Als uw platform alleen handmatige LTI 1.3-configuratie ondersteunt, gebruik dan de vier endpoints die in de Schoology-sectie hierboven zijn vermeld en neem contact op met de ondersteuning om dit af te ronden.