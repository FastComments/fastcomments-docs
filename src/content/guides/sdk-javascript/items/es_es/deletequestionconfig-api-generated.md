## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function executeDeletion() {
  const tenantId: string = "tenant_12345";
  const configId: string = "config_9876";
  const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(result);
}
executeDeletion();
[inline-code-end]

---