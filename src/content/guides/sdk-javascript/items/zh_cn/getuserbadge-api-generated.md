## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBadge() {
    const tenantId: string = "tenant-9f8b7c6d";
    const userId: string = "user-123456";
    const badgeResponse: APIGetUserBadgeResponse = await getUserBadge(tenantId, userId);
}
[inline-code-end]

---