## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| questionId | string | Нет |  |
| questionIds | Array<string> | Нет |  |
| urlId | string | Нет |  |
| startDate | Date | Нет |  |
| forceRecalculate | boolean | Нет |  |
| minValue | number | Нет |  |
| maxValue | number | Нет |  |
| limit | number | Нет |  |

## Ответ

Возвращает: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const questionId: string = 'q_98765';
const questionIds: string[] = ['q_98765', 'q_12345'];
const urlId: string = 'url_abc123';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0;
const maxValue: number = 5;
const limit: number = 50;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  questionId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
);
[inline-code-end]

---