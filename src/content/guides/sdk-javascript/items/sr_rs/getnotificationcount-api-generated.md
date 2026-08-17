## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| urlId | string | Не |  |
| fromCommentId | string | Не |  |
| viewed | boolean | Не |  |
| type | string | Не |  |

## Одговор

Враћа: [`GetNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoNotificationCount() {
    const tenantId: string = "tenant_001";
    const userId: string = "user_42";
    const urlId: string = "url_9f8e7d";
    const fromCommentId: string = "comment_12345";
    const viewed: boolean = false;
    const type: string = "mention";

    const result: GetNotificationCountResponse1 = await getNotificationCount(
        tenantId,
        userId,
        urlId,
        fromCommentId,
        viewed,
        type
    );

    console.log(result);
}

demoNotificationCount();
[inline-code-end]