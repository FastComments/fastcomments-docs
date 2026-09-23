## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返り値: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 例

[inline-code-attrs-start title = 'getQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "q-987654321";

const configResponse: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = configResponse.status;
const questionConfig?: QuestionConfig = configResponse.question;
const firstOption?: QuestionConfigCustomOptionsInner = questionConfig?.customOptions?.[0];
[inline-code-end]