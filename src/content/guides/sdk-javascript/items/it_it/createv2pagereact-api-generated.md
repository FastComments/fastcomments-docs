## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| id | string | Sì |  |
| title | string | No |  |

## Risposta

Restituisce: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_91f4b3b';
const urlId: string = 'https://news.site.com/articles/2026/06/fastcomments-integration';
const id: string = 'react_5f2c1a';
const title: string = 'FastComments Integration — June 2026';

const reactionWithTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id, title);
const reactionWithoutTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id);
[inline-code-end]