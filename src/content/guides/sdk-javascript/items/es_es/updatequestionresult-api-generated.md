## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'updateQuestionResult Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1f5e8b2-9a4d-4f3a-8d2e-6b7c9d0e1f2a";
const questionId: string = "qstn_1234567890";

const updateBody: UpdateQuestionResultBody = {
  score: 85,
  comment: "Adjusted based on new criteria",
  meta: [
    { key: "reviewer", value: "john.doe@example.com" },
    { key: "timestamp", value: new Date().toISOString() }
  ]
};

const response: APIEmptyResponse = await updateQuestionResult(tenantId, questionId, updateBody);
[inline-code-end]

---