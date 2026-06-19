---
Масова інформація про користувачів для тенанта. За заданими userIds повертає відображувану інформацію з User / SSOUser.
Використовується віджетом коментарів для доповнення користувачів, які щойно з'явилися через подію присутності.
Немає контексту сторінки: приватність застосовується однаково (приватні профілі приховано).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo only requires tenantId and ids; optional parameters are not applicable here.
[inline-code-end]

---