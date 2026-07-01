## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | boolean | 否 |  |
| type | string | 否 |  |

## 响应

返回：[`GetNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoNotificationCount() {
    const tenantId: string = "tenant_001";
    const userId: string = "user_42";
    const urlId: string = "url_9f8e7d";
    const fromCommentId: string = "comment_12345";
    const viewed: boolean = false;
    const type: string = "mention";

    const result: GetNotificationCountResponse1 = await getNotificationCount(
        tenantId,
        userId,
        urlId,
        fromCommentId,
        viewed,
        type
    );

    console.log(result);
}

demoNotificationCount();
[inline-code-end]

---