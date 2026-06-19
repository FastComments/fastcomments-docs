## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c2a';
const configId: string = 'questioncfg_4d3e2a1b';

const response: GetQuestionConfigResponse = await getQuestionConfig(tenantId, configId);

const questionConfig: QuestionConfig | undefined = response.questionConfig;
const customOptions: QuestionConfigCustomOptionsInner[] | undefined = questionConfig?.customOptions;
[inline-code-end]

---