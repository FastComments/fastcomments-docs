---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| limit | number | Ні |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressListResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant-01";
  const userId: string | undefined = "user-9e12b4";
  const limit: number | undefined = 20;
  const skip: number | undefined = 0;
  const result: APIGetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  console.log(result);
})();
[inline-code-end]

---