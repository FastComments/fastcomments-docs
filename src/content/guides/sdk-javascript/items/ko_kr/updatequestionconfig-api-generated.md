## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 예 |  |

## 응답

반환값: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "5a5c8f6e8f6e8f6e8f6e8f6e";

const updateBody: UpdateQuestionConfigBody = {
  question: "How satisfied are you with our service?",
  type: "multiple_choice",
  required: true,
  order: 1,
  customOptions: [
    { label: "Very satisfied", value: "5" },
    { label: "Neutral", value: "3" },
    { label: "Very dissatisfied", value: "1" }
  ],
  subQuestions: [
    {
      question: "What can we improve?",
      type: "text",
      condition: {
        operator: "matches",
        optionValue: "1"
      }
    }
  ]
};

const response: APIEmptyResponse = await updateQuestionConfig(
  tenantId,
  "q-config-98765",
  updateBody
);
[inline-code-end]