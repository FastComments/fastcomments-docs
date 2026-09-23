## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExamples() {
  const tenantId: string = "tenant_9f8b7c6d";
  const urlId: string = "blog/post-2024-06-15";
  const ssoToken: string = "sso_user_42";

  const resultWithSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId, ssoToken);
  const resultWithoutSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
}
[inline-code-end]