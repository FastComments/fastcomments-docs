## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_84b2";
const id: string = "question_9f3a";
const updateQuestionResultBody: UpdateQuestionResultBody = {
  outcome: "accepted",
  confidence: 0.88,
  moderatorId: "moderator_17",
  notes: "Validated by automated review" // 包含可选字段
};
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---