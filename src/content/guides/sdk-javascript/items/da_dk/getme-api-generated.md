---
Identificerer den legitimationsoplysning i brug: den lejer den tilhører, og for OAuth‑tokens, brugeren der har autoriseret den.  
Integrationer bruger dette til at teste en forbindelse og mærke den.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Svar

Returnerer: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getMe Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]

---