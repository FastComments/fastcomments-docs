---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| value | string | No |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getSearchUsers Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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