Identificira uporabljeno poverilnico: najemnika, kateremu pripada, in za OAuth žetone tudi uporabnika, ki ga je odobril.  
Integracije to uporabljajo za testiranje povezave in njeno označevanje.

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |

## Odgovor

Vrne: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]