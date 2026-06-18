## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a7c9';
const questionId: string = 'q_8d4f1b2c3a';
const options: { includeMeta?: boolean } = { includeMeta: true }; // демонстрация необязательного параметра
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, questionId);
const apiStatus: APIStatus | undefined = (result as unknown as { apiStatus?: APIStatus }).apiStatus;
const question: QuestionResult | undefined = (result as unknown as { question?: QuestionResult }).question;
[inline-code-end]

---