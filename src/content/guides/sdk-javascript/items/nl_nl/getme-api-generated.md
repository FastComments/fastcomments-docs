Identificeert de gebruikte referentie: de tenant waartoe deze behoort en, voor OAuth‑tokens, de gebruiker die deze heeft geautoriseerd.  
Integraties gebruiken dit om een verbinding te testen en te labelen.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |

## Reactie

Retourneert: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getMe Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]