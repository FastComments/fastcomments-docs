## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| urlId | string | Ne |  |
| fromCommentId | string | Ne |  |
| viewed | boolean | Ne |  |
| type | string | Ne |  |
| skip | number | Ne |  |

## Odziv

Vrne: [`GetNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse1.ts)

## Primer

[inline-code-attrs-start title = 'getNotifications Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---