## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## レスポンス

Returns: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'getSubscriptions の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const userId: string = "user-789";

    const responseWithUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
    const responseWithoutUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
})();
[inline-code-end]