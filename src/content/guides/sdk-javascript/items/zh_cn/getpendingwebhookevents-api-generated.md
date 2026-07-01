## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 否 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| type | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | number | 否 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetPendingWebhookEventsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getPendingWebhookEvents 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPendingEvents(): Promise<void> {
    const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
    const commentId: string = "cmt-987654321";
    const externalId: string = "ext-abc-123";
    const eventType: string = "comment_created";
    const type: string = "outbound";
    const domain: string = "myblog.com";
    const attemptCountGT: number = 3;
    const skip: number = 0;

    const pending: GetPendingWebhookEventsResponse1 = await getPendingWebhookEvents(
        tenantId,
        commentId,
        externalId,
        eventType,
        type,
        domain,
        attemptCountGT,
        skip
    );

    console.log(pending);
}

fetchPendingEvents();
[inline-code-end]