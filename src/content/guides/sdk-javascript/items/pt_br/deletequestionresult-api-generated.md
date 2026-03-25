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
const tenantId: string = "org-82479";
const questionId: string = "q-6a3d2e1f";
const includeArchived?: boolean = false; // parâmetro opcional demonstrando seleção alternativa de destino
const targetId: string = includeArchived ? "q-archived-112233" : questionId;
const result: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, targetId);
[inline-code-end]

---