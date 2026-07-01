## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| badgeId | string | 是 |  |
| userId | string | 否 |  |
| commentId | string | 否 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`PutAwardBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutAwardBadgeResponse.ts)

## 示例

[inline-code-attrs-start title = 'putAwardBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const badgeId: string = "badge-superstar"
    const userId: string = "user-42"
    const commentId: string = "comment-7f9c3"
    const broadcastId: string = "broadcast-2023-09"
    const result: PutAwardBadgeResponse = await putAwardBadge(badgeId, userId, commentId, broadcastId)
    console.log(result)
})()
[inline-code-end]