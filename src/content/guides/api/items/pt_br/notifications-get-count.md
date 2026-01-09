[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Esta rota retorna um objeto contendo o número de notificações no parâmetro `count`.

É mais lento que `/notification-count/` e custa o dobro de créditos, mas permite filtragem por mais dimensões.

Você pode filtrar pelos mesmos parâmetros do endpoint `/notifications`, como `userId`. Com SSO, o id do usuário está no formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Exemplo cURL de Contagem de Notificações Não Lidas para Usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL de Contagem de Notificações Não Lidas para Página Específica'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Contagem de Notificações'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtrar por usuário. **/
    userId?: string
    /** Filtrar por urlId. **/
    urlId?: string
    /** Filtrar pelo comentário de origem. **/
    fromCommentId?: string
    /** Filtrar por lido/não lido. **/
    viewed?: 'true' | 'false'
    /** Filtrar por tipo. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Contagem de Notificações'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    count?: number
}
[inline-code-end]