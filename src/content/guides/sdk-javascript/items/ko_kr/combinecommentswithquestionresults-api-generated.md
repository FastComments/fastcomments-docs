## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니요 |  |
| questionIds | Array<string> | 아니요 |  |
| urlId | string | 아니요 |  |
| startDate | Date | 아니요 |  |
| forceRecalculate | boolean | 아니요 |  |
| minValue | number | 아니요 |  |
| maxValue | number | 아니요 |  |
| limit | number | 아니요 |  |

## 응답

반환: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2';
const questionIds: string[] = ['question-42', 'question-43'];
const urlId: string = 'url_5d7b';
const startDate: Date = new Date('2026-02-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 1;
const maxValue: number = 5;
const limit: number = 50;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  undefined,
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