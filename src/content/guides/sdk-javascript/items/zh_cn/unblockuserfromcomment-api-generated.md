## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'unBlockUserFromComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f3e1';
const id: string = 'comment_9b2a4f';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'Reviewed by moderation team — reinstated',
  moderatorId: 'mod_21',
  unblockedAt: new Date().toISOString()
};
const userId: string = 'user_42';
const anonUserId: string = 'anon_e7f9';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
[inline-code-end]

---