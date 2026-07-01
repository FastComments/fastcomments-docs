## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| value | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## レスポンス

戻り値: [`GetSearchUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchUsersResponse.ts)

## 例

[inline-code-attrs-start title = 'getSearchUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSearch() {
    const query: string = "john.doe@example.com";
    const tenantId: string = "tenant_12345";
    const ssoToken: string = "sso_token_abc";

    const resultWithSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId, ssoToken);
    const resultWithoutSso: GetSearchUsersResponse = await getSearchUsers(query, tenantId);
}
[inline-code-end]