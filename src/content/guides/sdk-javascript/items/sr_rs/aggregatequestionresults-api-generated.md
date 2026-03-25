## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| questionId | string | Не |  |
| questionIds | Array<string> | Не |  |
| urlId | string | Не |  |
| timeBucket | AggregateTimeBucket | Не |  |
| startDate | Date | Не |  |
| forceRecalculate | boolean | Не |  |

## Одговор

Враћа: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример aggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // questionId изостављен, користи се questionIds уместо тога
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---