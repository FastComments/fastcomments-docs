## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const id: string = "user_98765";
  const response: GetUserResponse = await getUser(tenantId, id);
  const { user }: { user?: User } = response;
}
main();
[inline-code-end]

---