## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| questionId | string | 否 |  |
| questionIds | Array<string> | 否 |  |
| urlId | string | 否 |  |
| startDate | Date | 否 |  |
| forceRecalculate | boolean | 否 |  |
| minValue | number | 否 |  |
| maxValue | number | 否 |  |
| limit | number | 否 |  |

## 回應

回傳: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## 範例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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