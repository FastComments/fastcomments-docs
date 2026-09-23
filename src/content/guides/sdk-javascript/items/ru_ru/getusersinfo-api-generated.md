---
Информация о пользователях в bulk для арендатора. По заданным userIds возвращает отображаемую информацию из User / SSOUser.  
Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие присутствия.  
Без контекста страницы: конфиденциальность применяется единообразно (приватные профили маскируются).

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Response

Возвращает: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---