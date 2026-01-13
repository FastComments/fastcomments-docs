## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| urlId | string | Не |  |
| fromCommentId | string | Не |  |
| viewed | boolean | Не |  |
| type | string | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Пример

[inline-code-attrs-start title = 'getNotifications Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b1a2c';
  const userId: string = 'user_42';
  const urlId: string = 'https://news.example.com/articles/2026/01/11/comment-thread';
  const fromCommentId: string = 'cmt_9a7b';
  const viewed: boolean = false;
  const type: string = 'mention';
  const skip: number = 0;
  const response: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
  console.log(response);
})();
[inline-code-end]

---