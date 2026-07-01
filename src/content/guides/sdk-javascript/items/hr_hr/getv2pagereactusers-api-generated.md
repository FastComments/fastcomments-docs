## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vraća: [`GetV2PageReactUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getV2PageReactUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchReactUsers() {
  const tenantId: string = 'tenant_12345';
  const urlId: string = 'article-9876';
  const id: string = 'user_abcde';
  const response: GetV2PageReactUsersResponse1 = await getV2PageReactUsers(tenantId, urlId, id);
  console.log(response);
}
fetchReactUsers();
[inline-code-end]

---