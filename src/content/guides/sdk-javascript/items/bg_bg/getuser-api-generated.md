## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`GetUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_98765";
  const result: GetUserResponse1 = await getUser(tenantId, userId);
})();
[inline-code-end]

---