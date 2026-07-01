## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## 回應

返回：[`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSubscriptions 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const userId: string = "user-789";

    const responseWithUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
    const responseWithoutUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
})();
[inline-code-end]

---