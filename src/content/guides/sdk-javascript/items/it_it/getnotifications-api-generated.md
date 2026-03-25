## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_84b3f2";
const userId: string = "user_1279";
const urlId: string = "https://www.example.com/articles/2026/03/25/new-feature";
const fromCommentId: string = "cmt_5421";
const viewed: boolean = false;
const type: string = "mention";
const skip: number = 0;
const notifications: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
[inline-code-end]

---