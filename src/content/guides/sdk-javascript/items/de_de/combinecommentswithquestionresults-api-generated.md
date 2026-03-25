## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| questionId | string | Nein |  |
| questionIds | Array<string> | Nein |  |
| urlId | string | Nein |  |
| startDate | Date | Nein |  |
| forceRecalculate | boolean | Nein |  |
| minValue | number | Nein |  |
| maxValue | number | Nein |  |
| limit | number | Nein |  |

## Antwort

Rückgabe: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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