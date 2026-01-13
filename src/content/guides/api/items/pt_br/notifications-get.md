[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Esta rota retorna até 30 objetos `Notification` ordenados por `createdAt`, do mais novo para o mais antigo.

Você pode filtrar por `userId`. Com SSO, o id do usuário tem o formato `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Exemplo cURL: Notificações não lidas para usuário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Notificações'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginar pulando registros. **/
    skip?: number
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

[inline-code-attrs-start title = 'Estrutura da Resposta de Notificações'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---