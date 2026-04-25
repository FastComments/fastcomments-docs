## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | Array<string> | 아니오 |  |
| urlId | string | 아니오 |  |
| startDate | Date | 아니오 |  |
| forceRecalculate | boolean | 아니오 |  |
| minValue | number | 아니오 |  |
| maxValue | number | 아니오 |  |
| limit | number | 아니오 |  |

## 응답

반환: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-acme-001';
const questionId: string | undefined = 'q-analytics-42';
const questionIds: string[] | undefined = ['q-analytics-42', 'q-feedback-17'];
const urlId: string | undefined = 'url-987654';
const startDate: Date | undefined = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean | undefined = true;
const minValue: number | undefined = 1;
const maxValue: number | undefined = 5;
const limit: number | undefined = 250;
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