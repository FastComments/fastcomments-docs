Массовая информация о пользователях для tenant. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие presence.
Контекст страницы отсутствует: конфиденциальность применяется одинаково (приватные профили маскируются).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Ответ

Возвращает: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // необязательно; если undefined, по умолчанию запятая
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---