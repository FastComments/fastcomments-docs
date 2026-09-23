---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 예시

[inline-code-attrs-start title = 'getQuestionConfig 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "q-987654321";

const configResponse: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = configResponse.status;
const questionConfig?: QuestionConfig = configResponse.question;
const firstOption?: QuestionConfigCustomOptionsInner = questionConfig?.customOptions?.[0];
[inline-code-end]

---