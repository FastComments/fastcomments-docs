## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteV2PageReact Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // Aufruf ohne optionales sso
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // Aufruf mit optionalem sso
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]