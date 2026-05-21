## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| startDate | string | Non |  |
| questionId | string | Non |  |
| questionIds | string | Non |  |
| skip | number | Non |  |

## Réponse

Retourne : [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9b3f";
  const urlId: string = "survey-2026-spring";
  const userId: string = "user_00123";
  const startDate: string = "2026-04-01T00:00:00Z";
  const questionIds: string = "q_42,q_43";
  const skip: number = 0;
  const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
  console.log(result);
})();
[inline-code-end]

---