## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| value | string | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetSearchUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchUsersResponse.ts)

## Primer

[inline-code-attrs-start title = 'getSearchUsers Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSearch() {
    const query: string = "john.doe@example.com";
    const tenantId: string = "tenant_12345";
    const ssoToken: string = "sso_token_abc";

    const resultWithSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId, ssoToken);
    const resultWithoutSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId);
}
[inline-code-end]