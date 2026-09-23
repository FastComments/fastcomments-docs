## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getQuestionConfig Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "q-987654321";

const configResponse: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = configResponse.status;
const questionConfig?: QuestionConfig = configResponse.question;
const firstOption?: QuestionConfigCustomOptionsInner = questionConfig?.customOptions?.[0];
[inline-code-end]