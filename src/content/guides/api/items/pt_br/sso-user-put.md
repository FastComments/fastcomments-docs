[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Esta rota permite atualizar um único usuário SSO.

[inline-code-attrs-start title = 'Exemplo de cURL para atualização de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Neste exemplo especificamos `groupIds` para controle de acesso, mas isso é opcional.

[inline-code-attrs-start title = 'Estrutura de Requisição de Atualização de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Quando o email ou username for alterado, você pode definir isto como true para também atualizar os comentários do usuário. Isto dobrará o custo em créditos. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Atualização de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Incluído em caso de falha. **/
    reason?: string
    user?: SSOUser; // Retornamos o usuário atualizado em caso de sucesso.
}
[inline-code-end]