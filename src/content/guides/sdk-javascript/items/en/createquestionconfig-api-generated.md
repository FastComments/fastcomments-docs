## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## Response

Returns: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse.ts)

## Example

[inline-code-attrs-start title = 'createQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'demo_fastcomments';

const questionConfigBody: CreateQuestionConfigBody = {
  label: 'How would you rate this article?',
  type: 'RATING'
};

const customOptions: QuestionConfigCustomOptionsInner[] = [
  { label: 'Very helpful', score: 5 },
  { label: 'Not helpful', score: 1 }
];

const subQuestionCondition: SubQuestionCondition = {
  questionId: 'q-101',
  operator: 'EQUALS' as QuestionConditionOperator,
  value: 5
};

const questionConfig: QuestionConfig = {
  id: 'qc-8842',
  tenantId,
  body: questionConfigBody,
  customOptions,
  condition: {
    operator: 'GREATER_THAN' as QuestionConditionOperator,
    subQuestionCondition
  } as QuestionCondition
};

const response: CreateQuestionConfigResponse = await createQuestionConfig(tenantId, {
  label: questionConfig.body.label,
  type: questionConfig.body.type,
  customOptions,
  condition: questionConfig.condition
});

const status: APIStatus = response.status;
const createdConfigId: string = response.questionConfig.id;
console.log(`Question config created: ${createdConfigId} (${status})`);
[inline-code-end]
