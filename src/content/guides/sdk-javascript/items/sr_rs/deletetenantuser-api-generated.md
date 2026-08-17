## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Одговор

Враћа: [`DeleteTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantUserResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteTenantUser Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoDeleteTenantUser() {
  const tenantId: string = "acme-corp-tenant";
  const userId: string = "user-9876";

  // Обришите корисника и све његове коментаре, користећи режим трајног брисања
  const resultWithOptions: DeleteTenantUserResponse = await deleteTenantUser(
    tenantId,
    userId,
    "true",
    "hard"
  );

  // Обришите корисника без уклањања коментара (подразумевано понашање)
  const resultBasic: DeleteTenantUserResponse = await deleteTenantUser(tenantId, userId);
}
[inline-code-end]

---