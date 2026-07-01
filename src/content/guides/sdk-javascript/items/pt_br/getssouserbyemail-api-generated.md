## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| email | string | Sim |  |

## Resposta

Retorna: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByEmailAPIResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getSSOUserByEmail'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchSSOUser() {
  const tenantId: string = "tenant_12345";
  const email: string = "john.doe@example.com";

  const result: GetSSOUserByEmailAPIResponse = await getSSOUserByEmail(tenantId, email);
  const user: APISSOUser | undefined = result?.user;
}
[inline-code-end]

---