## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| value | string | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Returns: [`GetSearchUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchUsersResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getSearchUsers Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSearch() {
    const query: string = "john.doe@example.com";
    const tenantId: string = "tenant_12345";
    const ssoToken: string = "sso_token_abc";

    const resultWithSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId, ssoToken);
    const resultWithoutSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId);
}
[inline-code-end]