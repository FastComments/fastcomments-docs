---
Зведена інформація про користувачів для tenant. За заданими userIds повертає відображувану інформацію з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: конфіденційність застосовується однаково (приватні профілі замасковані).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| ids | string | Так |  |

## Відповідь

Повертає: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // необов'язково; якщо undefined, за замовчуванням використовується кома
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---