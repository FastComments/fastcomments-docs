## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExamples() {
  const tenantId: string = "tenant_9f8b7c6d";
  const urlId: string = "blog/post-2024-06-15";
  const ssoToken: string = "sso_user_42";

  const resultWithSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId, ssoToken);
  const resultWithoutSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
}
[inline-code-end]