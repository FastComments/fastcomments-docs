## Parameters

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| id | string | Sí |  |
| sso | string | No |  |

## Response

Devuelve: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo deleteV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // Llamada sin sso opcional
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // Llamada con sso opcional
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]