## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | boolean | 否 |  |
| type | string | 否 |  |

## 回應

返回: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-12345";
const urlId: string = "https://app.example.com/dashboard";
const fromCommentId: string = "cmt-9876";
const viewed: boolean = false;
const type: string = "reply";

const notificationCount: GetNotificationCountResponse = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]

---