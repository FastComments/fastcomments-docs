## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| questionId | string | Не |  |
| questionIds | Array<string> | Не |  |
| urlId | string | Не |  |
| startDate | Date | Не |  |
| forceRecalculate | boolean | Не |  |
| minValue | number | Не |  |
| maxValue | number | Не |  |
| limit | number | Не |  |

## Отговор

Връща: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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