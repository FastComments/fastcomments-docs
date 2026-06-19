## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| title | string | Não |  |

## Resposta

Retorna: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const urlId: string = 'blog/how-we-reduce-latency';
const title: string | undefined = 'Reducing Frontend Latency with FastComments';
const createResponse: CreateV1PageReact = await createV1PageReact(tenantId, urlId, title);
const createResponseNoTitle: CreateV1PageReact = await createV1PageReact(tenantId, urlId);
[inline-code-end]

---