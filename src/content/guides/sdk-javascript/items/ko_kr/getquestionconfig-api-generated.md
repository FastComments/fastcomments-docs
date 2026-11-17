## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환값: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const questionId: string = 'poll-question-789';

const response: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = response.status;
const config: QuestionConfig = response.config;

const type: string = config.type;
const customOptions: QuestionConfigCustomOptionsInner[] = config.customOptions || [];

if (customOptions.length > 0) {
  const firstOptionLabel: string | undefined = customOptions[0].label;
  console.log(`Config ${config.id} for tenant ${tenantId}, first option: ${firstOptionLabel}`);
}
[inline-code-end]