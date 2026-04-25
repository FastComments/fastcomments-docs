## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantIdEnv: string | undefined = process.env.FASTCOMMENTS_TENANT_ID;
const tenantId: string = tenantIdEnv ?? 'tenant_78b3f2';
const id: string = 'qres-9f2a3b1c';
const response: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
[inline-code-end]

---