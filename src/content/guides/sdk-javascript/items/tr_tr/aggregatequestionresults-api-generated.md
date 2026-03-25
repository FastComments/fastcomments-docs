## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| questionId | string | Hayır |  |
| questionIds | Array<string> | Hayır |  |
| urlId | string | Hayır |  |
| timeBucket | AggregateTimeBucket | Hayır |  |
| startDate | Date | Hayır |  |
| forceRecalculate | boolean | Hayır |  |

## Yanıt

Döndürür: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Örnek

[inline-code-attrs-start title = 'aggregateQuestionResults Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_98765';
const questionIds: Array<string> = ['q-102', 'q-103'];
const urlId: string = 'url_55b3';
const timeBucket: AggregateTimeBucket = { unit: 'day', size: 7 };
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;

const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
  tenantId,
  undefined, // questionId atlandı, bunun yerine questionIds kullanılıyor
  questionIds,
  urlId,
  timeBucket,
  startDate,
  forceRecalculate
);
[inline-code-end]

---