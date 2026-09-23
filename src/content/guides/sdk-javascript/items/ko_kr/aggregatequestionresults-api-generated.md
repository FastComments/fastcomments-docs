## 매개변수

| 이름 | 유형 | 필수 | 설명 |
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

## 예시

[inline-code-attrs-start title = 'aggregateQuestionResults 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runAggregation() {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const questionId: string = 'question-9f8e7d6c5b4a3';
  const questionIds: string[] = ['question-1a2b3c', 'question-4d5e6f'];
  const urlId: string = 'blog-post-2023-08-15';
  const startDate: Date = new Date('2023-01-01T00:00:00Z');
  const forceRecalculate: boolean = true;

  const result: AggregateQuestionResultsResponse = await aggregateQuestionResults(
    tenantId,
    questionId,
    questionIds,
    urlId,
    undefined,
    startDate,
    forceRecalculate
  );
}
[inline-code-end]

---