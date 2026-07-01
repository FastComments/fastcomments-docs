---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | boolean | 否 |  |
| type | string | 否 |  |
| skip | number | 否 |  |

## 回應

返回: [`GetNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getNotifications 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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