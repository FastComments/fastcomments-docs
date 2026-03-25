## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| questionId | string | לא |  |
| questionIds | Array<string> | לא |  |
| urlId | string | לא |  |
| startDate | Date | לא |  |
| forceRecalculate | boolean | לא |  |
| minValue | number | לא |  |
| maxValue | number | לא |  |
| limit | number | לא |  |

## תגובה

מחזיר: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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