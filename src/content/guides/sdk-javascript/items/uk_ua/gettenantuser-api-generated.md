## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Відповідь

Повертає: [`GetTenantUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser(): Promise<void> {
  const tenantId: string = "tenant-01a2b3c";
  const userId: string = "user-7890";
  const result: GetTenantUserResponse1 = await getTenantUser(tenantId, userId);
  console.log(result);
}
[inline-code-end]