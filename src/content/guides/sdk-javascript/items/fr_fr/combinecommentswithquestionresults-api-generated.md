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

Renvoie : [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7c9f4b3a';
const questionIds: string[] = ['q-4f8b2a1c', 'q-9d3e7b0f'];
const urlId: string = 'url_93b2c1a7';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0.2;
const maxValue: number = 0.95;
const limit: number = 100;

const combinedResult: CombineQuestionResultsWithCommentsResponse = await combineCommentsWithQuestionResults({
  tenantId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
});
[inline-code-end]

---