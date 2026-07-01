## 매개변수

| 이름 | 유형 | 필수 | 설명 |
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

반환: [`CombineCommentsWithQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResultsResponse.ts)

## 예시

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const questionIds: string[] = ["question_1", "question_2"];
const urlId: string = "article-9876";
const startDate: Date = new Date("2023-01-01T00:00:00Z");
const forceRecalculate: boolean = false;
const minValue: number = 1;
const maxValue: number = 5;
const limit: number = 50;

const result: CombineCommentsWithQuestionResultsResponse = await combineCommentsWithQuestionResults(
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