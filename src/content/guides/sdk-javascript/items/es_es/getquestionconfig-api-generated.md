## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'getQuestionConfig Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-42";
const questionId: string = "question-7f9b3e";

const response: GetQuestionConfigResponse1 = await getQuestionConfig(tenantId, questionId);
[inline-code-end]