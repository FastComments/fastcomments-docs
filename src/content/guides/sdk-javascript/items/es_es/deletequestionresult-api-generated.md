---
## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'deleteQuestionResult Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDelete() {
  const tenantId: string = "tenant_12345";
  const resultId: string = "qr_98765";

  const response: APIEmptyResponse = await deleteQuestionResult(tenantId, resultId);
  console.log(response);
}
[inline-code-end]

---