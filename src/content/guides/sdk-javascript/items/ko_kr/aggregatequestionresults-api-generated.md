## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | Array<string> | 아니오 |  |
| urlId | string | 아니오 |  |
| timeBucket | AggregateTimeBucket | 아니오 |  |
| startDate | Date | 아니오 |  |
| forceRecalculate | boolean | 아니오 |  |

## 응답

반환: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse.ts)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a4f7c2b';
const questionIds: string[] = ['q_101', 'q_102', 'q_103'];
const urlId: string = 'url_3f9b12';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const result: AggregateQuestionResultsResponse = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, forceRecalculate);
[inline-code-end]

---