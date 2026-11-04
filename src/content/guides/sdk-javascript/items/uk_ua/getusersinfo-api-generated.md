Масова інформація про користувачів для орендаря. За заданими userIds повертає інформацію для відображення з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: конфіденційність застосовується однаково (приватні профілі маскуються).

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| ids | string | Так |  |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]