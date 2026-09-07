[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Esta API usa paginação, fornecida pelos parâmetros `skip`, `limit`, `before` e `after`. AuditLogs são retornados em páginas de `1000` por padrão, até um `limit` máximo de `10000`, ordenados por `when` e `id`. As páginas são grandes porque este endpoint geralmente é usado para exportar o histórico em vez de paginar interativamente.

Cada `100` logs retornados tem um custo de crédito de `1`.

Por padrão, você receberá uma lista com **os itens mais recentes primeiro**. Dessa forma, você pode fazer polling começando com `skip=0`, paginando até encontrar o último registro que consumiu.

Alternativamente, você pode ordenar do mais antigo para o mais recente e paginar até que não haja mais registros.

A ordenação pode ser feita definindo `order` como `ASC` ou `DESC`. O padrão é `DESC`.

Consultar por data é possível via `before` e `after` como timestamps em milissegundos. `before` e `after` NÃO são inclusivos, e cada um pode ser usado isoladamente.

## Encontrando o que aconteceu com uma pessoa

Cada evento registra quem o realizou (`username`, `userId`, `ip`) e, separadamente, sobre o que ele foi realizado. `targetLabel` é um rótulo legível para aquele objeto, por exemplo `jsmith (jsmith@example.com)`, e `targetId` é seu id. Use `target` para uma correspondência de substring sem distinção entre maiúsculas e minúsculas no rótulo quando você conhece o nome ou e‑mail de uma pessoa, mas não seu id.

Exclusões capturam o rótulo no momento do evento, de modo que um usuário ou moderador removido ainda pode ser identificado após o registro subjacente ser excluído.

## Inquilinos gerenciados

Se o seu inquilino gerencia outros inquilinos, defina `includeManagedTenants=true` para retornar eventos do seu inquilino e de todos os inquilinos que ele gerencia em uma única resposta. O `tenantId` de cada log retornado indica de qual inquilino ele provém.

[inline-code-attrs-start title = 'Exemplo cURL de AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]