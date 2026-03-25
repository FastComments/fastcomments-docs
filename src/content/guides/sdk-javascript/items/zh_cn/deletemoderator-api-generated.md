## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| sendEmail | string | 否 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deleteModerator 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2d4a6c';
const moderatorId: string = 'moderator_84a1b9c2';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
[inline-code-end]

---