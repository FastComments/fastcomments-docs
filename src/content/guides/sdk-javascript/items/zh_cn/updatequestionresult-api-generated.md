## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f2b9';
const id: string = 'question_3a7c1d';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  result: { verdict: 'helpful', confidence: 0.92 },
  reviewer: { id: 'mod_102', name: 'Aisha Rahman' },
  notifyUser: true // 包含可选参数
};
const response: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---