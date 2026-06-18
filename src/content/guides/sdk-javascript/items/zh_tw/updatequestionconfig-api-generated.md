## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 是 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateQuestionConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f3b8a2e-4c6d-4b4f-a1b2-9e8f7d6c5b3a";
const id: string = "q_7c2e1b4a-5d6f-4a8b-9c3d-2e1f0b9a4c5d";
const options: QuestionConfigCustomOptionsInner[] = [
  { value: "1", label: "Very dissatisfied" },
  { value: "2", label: "Dissatisfied" },
  { value: "3", label: "Neutral" },
  { value: "4", label: "Satisfied" },
  { value: "5", label: "Very satisfied" }
];
const updateQuestionConfigBody: UpdateQuestionConfigBody = {
  label: "How satisfied are you with the article?",
  enabled: true,
  required: false, // 示範可選參數
  options
};
const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]