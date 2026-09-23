## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| value | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getSearchUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearch() {
  const tenantId: string = "tenant-9876";
  const emailFragment: string = "jane";
  const ssoToken: string = "sso-token-456";

  const resultWithAll: ModerationUserSearchResponse = await getSearchUsers(tenantId, emailFragment, ssoToken);
  const resultWithTenantOnly: ModerationUserSearchResponse = await getSearchUsers(tenantId);
}
[inline-code-end]

---