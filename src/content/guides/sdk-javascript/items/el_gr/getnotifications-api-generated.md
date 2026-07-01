## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |
| urlId | string | Όχι |  |
| fromCommentId | string | Όχι |  |
| viewed | boolean | Όχι |  |
| type | string | Όχι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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