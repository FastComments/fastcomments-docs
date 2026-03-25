## Parametri

| Ime | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createQuestionResultBody | CreateQuestionResultBody | Да |  |

## Одговор

Враћа: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Примјер

[inline-code-attrs-start title = 'Primjer createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-72b1f4";
const meta: MetaItem[] = [{ key: "platform", value: "web" }];
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "question-83472",
  commenterId: "user-5521",
  answers: [{ subQuestionId: "sq-1", value: "Yes" }],
  meta, // neобavezni метаподаци
  note: "Follow-up requested" // приказан необавезни параметар
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---