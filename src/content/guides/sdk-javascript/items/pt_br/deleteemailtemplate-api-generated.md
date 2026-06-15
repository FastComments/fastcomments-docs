## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9c4f1b2a";
const id: string = "emailtmpl_4d2b9a5e";
const requestorNote: string | undefined = undefined; // metadados opcionais (não exigido pela função)
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
[inline-code-end]

---