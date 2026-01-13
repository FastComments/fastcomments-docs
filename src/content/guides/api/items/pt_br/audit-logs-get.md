[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Esta API usa paginação, fornecida pelos parâmetros `skip`, `before` e `after`. AuditLogs são retornados em páginas de `1000`, ordenados por `when` e `id`.

Buscar cada `1000` logs tem um custo de crédito de `10`.

Por padrão, você receberá uma lista com **os itens mais novos primeiro**. Dessa forma, você pode consultar começando com `skip=0`, paginando até encontrar o último registro que consumiu.

Alternativamente, você pode ordenar do mais antigo para o mais novo e paginar até não haver mais registros.

A ordenação pode ser feita definindo `order` como `ASC` ou `DESC`. O padrão é `ASC`.

Consultar por data é possível via `before` e `after` como timestamps com milissegundos. `before` e `after` NÃO são inclusivos.

[inline-code-attrs-start title = 'Exemplo cURL do AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição do AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta do AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Os logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---