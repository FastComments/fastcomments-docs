## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| urlId | string | Ne |  |
| fromCommentId | string | Ne |  |
| viewed | boolean | Ne |  |
| type | string | Ne |  |

## Odgovor

Vraća: [`GetNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---