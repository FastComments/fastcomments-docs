## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| createQuestionResultBody | CreateQuestionResultBody | Да |  |

## Одговор

Враћа: [`CreateQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse1.ts)

## Пример

[inline-code-attrs-start title = 'createQuestionResult Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";

const metaItem: MetaItem = {
  key: "campaign",
  value: "spring-launch"
};

const questionResultBody: CreateQuestionResultBody = {
  questionId: "question-42",
  answer: "Positive",
  metadata: [metaItem]
  // опционална поља као што су белешке су изостављена
};

const result: CreateQuestionResultResponse1 = await createQuestionResult(tenantId, questionResultBody);
[inline-code-end]