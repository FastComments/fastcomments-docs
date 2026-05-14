#### Sakai

Sakai podpira dinamično registracijo LTI 1.3 v izdajah z LTI Advantage. Iz **Delovnega prostora administracije**:

1. Prijavite se kot skrbnik Sakai in odprite **Delovni prostor administracije**.
2. Izberite **Zunanji pripomočki** > **Namesti orodje LTI 1.3**.
3. Prilepite URL za registracijo FastComments in pošljite.
4. Potrdite orodje, ko se izmenjava roke (handshake) zaključi.

Orodje se nato pojavi pod **Zunanji pripomočki** in ga lahko vzdrževalci spletnih mest dodajo na strani.

#### Schoology

Instance Schoology Enterprise podpirajo LTI 1.3, vendar se razpoložljivost dinamične registracije razlikuje glede na namestitev. Posvetujte se z vašim upraviteljem računa Schoology.

Če dinamična registracija ni na voljo na vaši instanci Schoology, boste morali integracijo konfigurirati ročno z uporabo naslednjih končnih točk:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Ko vam Schoology dodeli Client ID in Deployment ID, se obrnite na podporo FastComments, da konfiguracijo registrirajo na vašem najemniku.

#### Druge platforme LTI 1.3

Kateri koli LMS, ki sledi specifikaciji IMS LTI 1.3 Advantage, bi moral delovati z istim URL-jem za registracijo. Poiščite nastavitev z oznako "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ali podobno.

Če vaša platforma podpira le ročno nastavitev LTI 1.3, uporabite štiri končne točke, navedene v razdelku Schoology zgoraj, in se obrnite na podporo, da dokončate postopek.