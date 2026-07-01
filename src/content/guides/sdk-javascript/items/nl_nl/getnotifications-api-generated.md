## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## Respons

Retourneert: [`GetNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";

  const notifications: GetNotificationsResponse1 = await getNotifications(tenantId, userId);
  console.log(notifications);

  const more: GetNotificationsResponse1 = await getNotifications(
    tenantId,
    undefined,
    "article-5678",
    undefined,
    true,
    "reply",
    10
  );
  console.log(more);
}
demo();
[inline-code-end]