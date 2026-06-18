---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니요 |  |
| questionIds | Array<string> | 아니요 |  |
| urlId | string | 아니요 |  |
| timeBucket | AggregateTimeBucket | 아니요 |  |
| startDate | Date | 아니요 |  |
| forceRecalculate | boolean | 아니요 |  |

## 응답

반환: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f2b3c';
const questionIds: Array<string> = ['q-7a1b2c', 'q-8d3e4f'];
const urlId: string = 'url_9f8e7d';
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const result: AggregateQuestionResults200Response = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, true);
[inline-code-end]

---