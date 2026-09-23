## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| deleteComments | string | Не |  |
| commentDeleteMode | string | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "u-112233";

  const result1: APIEmptyResponse = await deleteTenantUser(tenantId, userId);
  const result2: APIEmptyResponse = await deleteTenantUser(tenantId, userId, "true", "hard");

  console.log(result1, result2);
})();
[inline-code-end]

---