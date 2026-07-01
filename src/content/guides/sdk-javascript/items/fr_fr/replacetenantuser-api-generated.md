## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Réponse

Retourne : [`ReplaceTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateUser() {
  const tenantId: string = "c3d1f2e4-5b6a-4c7d-9f2e-1234567890ab";
  const userId: string = "u-654321";
  const replaceBody: ReplaceTenantUserBody = {
    email: "newuser@example.com",
    username: "newusername"
  };
  const response: ReplaceTenantUserResponse = await replaceTenantUser(
    tenantId,
    userId,
    replaceBody,
    "true"
  );
  console.log(response);
}
updateUser();
[inline-code-end]