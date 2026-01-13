## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| urlId | string | Nee |  |
| fromCommentId | string | Nee |  |
| viewed | boolean | Nee |  |
| type | string | Nee |  |

## Response

Geeft terug: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getNotificationCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a9b7c';
const userId: string = 'user_42b3c';
const urlId: string = 'https://blog.example.com/posts/introducing-new-editor';
const fromCommentId: string | undefined = undefined;
const viewed: boolean = false;
const type: string = 'mention';
const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type);
[inline-code-end]

---