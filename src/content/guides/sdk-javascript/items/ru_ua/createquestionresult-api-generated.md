---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createQuestionResultBody | CreateQuestionResultBody | Да |  |

## Ответ

Возвращает: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc-tenant-512';
const meta: MetaItem[] = [{ key: 'source', value: 'article' }];
const body: CreateQuestionResultBody = {
  questionId: 'q-94',
  userId: 'user_332',
  answers: [{ optionId: 'opt_a', score: 1 }],
  meta, // необязательные метаданные
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, body);
[inline-code-end]

---