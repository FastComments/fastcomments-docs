## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTenantUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_7f3b2a9c';
  const skip: number = 20; // optional parameter demonstrated
  const result: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
  console.log(result);
})();
[inline-code-end]

---