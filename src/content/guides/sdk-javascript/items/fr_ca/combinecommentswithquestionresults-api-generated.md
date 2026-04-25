## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| questionId | string | Non |  |
| questionIds | Array<string> | Non |  |
| urlId | string | Non |  |
| startDate | Date | Non |  |
| forceRecalculate | boolean | Non |  |
| minValue | number | Non |  |
| maxValue | number | Non |  |
| limit | number | Non |  |

## Réponse

Renvoie : [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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