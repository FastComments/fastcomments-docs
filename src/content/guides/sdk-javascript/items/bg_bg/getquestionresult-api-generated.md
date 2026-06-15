## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a7c9';
const questionId: string = 'q_8d4f1b2c3a';
const options: { includeMeta?: boolean } = { includeMeta: true }; // демонстрация на опционален параметър
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, questionId);
const apiStatus: APIStatus | undefined = (result as unknown as { apiStatus?: APIStatus }).apiStatus;
const question: QuestionResult | undefined = (result as unknown as { question?: QuestionResult }).question;
[inline-code-end]

---