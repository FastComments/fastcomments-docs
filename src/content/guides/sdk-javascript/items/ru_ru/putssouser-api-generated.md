## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | boolean | No |  |

## Ответ

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример putSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_abc123";
  const userId: string = "user_456def";

  const updateData: UpdateAPISSOUserData = {
    email: "jane.doe@example.com",
    displayName: "Jane Doe",
    isActive: true,
  };

  const response: PutSSOUserAPIResponse = await putSSOUser(tenantId, userId, updateData, true);
})();
[inline-code-end]