## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |
| urlId | string | Nein |  |
| fromCommentId | string | Nein |  |
| viewed | boolean | Nein |  |
| type | string | Nein |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'getNotifications Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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