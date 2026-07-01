## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| urlId | string | Sí |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`PutReopenThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutReopenThreadResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo putReopenThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage() {
  const urlId: string = "thread-9f8b7c6a";
  const tenantId: string = "tenant-001";
  const sso: string = "sso-3f9d2e1a";

  const resultAll: PutReopenThreadResponse = await putReopenThread(urlId, tenantId, sso);
  console.log(resultAll);

  const resultMinimal: PutReopenThreadResponse = await putReopenThread(urlId);
  console.log(resultMinimal);
}
[inline-code-end]