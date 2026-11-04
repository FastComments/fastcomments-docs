#### Sakai

Sakai podpira LTI 1.3 Dynamic Registration na izdajah z LTI Advantage. Iz Administration Workspace:

1. Prijavite se kot skrbnik Sakai in odprite **Administration Workspace**.
2. Izberite **External Tools** > **Install LTI 1.3 Tool**.
3. Prilepite registracijski URL FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) in oddajte.
4. Odobrite orodje, ko se rokovanje zaključi.

Orodje se nato prikaže pod **External Tools** in ga vzdrževalci lahko dodajo na svoja mesta.

#### Schoology

Instance Schoology Enterprise podpirajo LTI 1.3, vendar se razpoložljivost Dynamic Registration razlikuje glede na namestitev. Preverite pri svojem upravitelju računa Schoology.

Če Dynamic Registration ni na voljo v vaši instanci Schoology, boste morali integracijo nastaviti ročno z naslednjimi končnimi točkami:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Ko vam Schoology dodeli Client ID in Deployment ID, se obrnite na podporo FastComments, da registrirate konfiguracijo na vašem najemniku.

#### Druge platforme LTI 1.3

Vsak LMS, ki sledi specifikaciji IMS LTI 1.3 Advantage, bi moral delovati z istim registracijskim URL-jem (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>). Poiščite nastavitev z oznako "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ali podobno.

Če vaša platforma podpira samo ročno nastavitev LTI 1.3, uporabite štiri končne točke, navedene v razdelku Schoology zgoraj, in za dokončanje kontaktirajte podporo.