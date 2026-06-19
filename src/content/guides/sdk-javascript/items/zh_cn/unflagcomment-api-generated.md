## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回：[`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'unFlagComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const id: string = 'cmt-9b8f7d6a5';
const userId: string = 'user-42a7c9e1';

const result: FlagCommentResponse = await unFlagComment(tenantId, id, userId);
[inline-code-end]

---