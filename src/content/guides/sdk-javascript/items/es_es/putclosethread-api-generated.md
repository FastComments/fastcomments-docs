## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| urlId | string | Sí |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`PutCloseThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutCloseThreadResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo putCloseThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function closeThreadDemo(): Promise<void> {
  const urlId: string = "article-2023-09-15";
  const tenantId: string = "tenant-42";
  const sso: string = "sso-token-xyz";

  const response: PutCloseThreadResponse = await putCloseThread(urlId, tenantId, sso);
  console.log(response);
}

closeThreadDemo();
[inline-code-end]