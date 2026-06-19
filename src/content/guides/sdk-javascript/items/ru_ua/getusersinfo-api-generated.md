---
Групповая информация о пользователях для тенанта. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для обогащения данных пользователей, которые только что появились через событие присутствия.
Без контекста страницы: конфиденциальность применяется одинаково (частные профили скрыты).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Ответ

Возвращает: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo требует только tenantId и ids; необязательные параметры тут не применимы.
[inline-code-end]

---