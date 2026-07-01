## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## 回應

Returns: [`CreateQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse1.ts)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";

const metaItem: MetaItem = {
  key: "campaign",
  value: "spring-launch"
};

const questionResultBody: CreateQuestionResultBody = {
  questionId: "question-42",
  answer: "Positive",
  metadata: [metaItem]
  // optional fields such as notes are omitted
};

const result: CreateQuestionResultResponse1 = await createQuestionResult(tenantId, questionResultBody);
[inline-code-end]