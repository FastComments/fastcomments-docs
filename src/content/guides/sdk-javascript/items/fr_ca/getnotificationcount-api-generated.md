## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Non |  |
| fromCommentId | string | Non |  |
| viewed | boolean | Non |  |
| type | string | Non |  |

## Réponse

Retourne : [`GetNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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