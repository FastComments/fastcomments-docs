## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |

## Odgovor

Vraća: [`GetNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse1.ts)

## Primer

[inline-code-attrs-start title = 'getNotificationCount Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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