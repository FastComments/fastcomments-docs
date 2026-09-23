## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 是 |  |

## 响应

返回: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## 示例

[inline-code-attrs-start title = 'createModerator 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // 可选字段; 如果不需要可以省略
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]