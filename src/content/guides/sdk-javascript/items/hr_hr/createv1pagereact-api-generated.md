## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| title | string | Ne |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Primjer

[inline-code-attrs-start title = 'createV1PageReact Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const urlId: string = 'blog/how-we-reduce-latency';
const title: string | undefined = 'Reducing Frontend Latency with FastComments';
const createResponse: CreateV1PageReact = await createV1PageReact(tenantId, urlId, title);
const createResponseNoTitle: CreateV1PageReact = await createV1PageReact(tenantId, urlId);
[inline-code-end]

---