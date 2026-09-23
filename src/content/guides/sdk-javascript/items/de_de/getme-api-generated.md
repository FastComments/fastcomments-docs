Identifiziert das verwendete Anmeldeinformation: den Mandanten, zu dem es gehört, und bei OAuth-Token den Benutzer, der es autorisiert hat.  
Integrationen verwenden dies, um eine Verbindung zu testen und zu kennzeichnen.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Yes |  |

## Response

Rückgabe: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Example

[inline-code-attrs-start title = 'getMe Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]