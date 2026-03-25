## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| urlId | string | Nej |  |
| fromCommentId | string | Nej |  |
| viewed | boolean | Nej |  |
| type | string | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getNotifications Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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