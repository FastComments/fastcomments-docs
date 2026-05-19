#### Sakai

Sakai podpira LTI 1.3 Dynamic Registration v izdajah z LTI Advantage. V Administracijskem delovnem prostoru:

1. Prijavite se kot skrbnik Sakai in odprite **Administration Workspace**.
2. Izberite **External Tools** > **Install LTI 1.3 Tool**.
3. Prilepite FastComments URL za registracijo (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) in potrdite.
4. Potrdite orodje, ko se handshake zaključi.

Orodje se nato pojavi pod **External Tools** in ga lahko vzdrževalci strani dodajo na spletna mesta.

#### Schoology

Primere Schoology Enterprise podpirajo LTI 1.3, vendar se razpoložljivost Dynamic Registration razlikuje glede na namestitev. Posvetujte se z upraviteljem vašega Schoology računa.

Če Dynamic Registration ni na voljo v vaši Schoology namestitvi, boste morali integracijo konfigurirati ročno z uporabo teh končnih točk:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Ko vam Schoology dodeli Client ID in Deployment ID, se obrnite na podporo FastComments, da konfiguracijo registrirajo na vašem najemniku.

#### Other LTI 1.3 Platforms

Kateri koli LMS, ki sledi specifikaciji IMS LTI 1.3 Advantage, bi moral delovati z enakim URL-jem za registracijo (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>). Poiščite nastavitev z oznako "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ali podobno.

Če vaša platforma podpira samo ročno nastavitev LTI 1.3, uporabite štiri končne točke, navedene v razdelku Schoology zgoraj, in se obrnite na podporo za dokončanje.