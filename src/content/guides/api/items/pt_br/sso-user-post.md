[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a criação de um único usuário SSO.

Tentar criar dois usuários com o mesmo ID resultará em um erro.

[inline-code-attrs-start title = 'Exemplo cURL de Criação de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Neste exemplo especificamos `groupIds` para controle de acesso, mas isso é opcional.

[inline-code-attrs-start title = 'Estrutura de Requisição de Criação de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Criação de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Incluído em caso de falha. **/
    reason?: string
    user?: SSOUser; // Retornamos o usuário criado em caso de sucesso.
}
[inline-code-end]

#### Nota de Integração

Os dados enviados pela API podem ser sobrescritos simplesmente enviando um payload HMAC de SSO User diferente. Por exemplo, se
você definir um username via API, mas então enviar um diferente através do fluxo SSO ao carregar a página, nós atualizaremos
automaticamente o username deles.

Não atualizaremos parâmetros do usuário nesse fluxo a menos que você os especifique explicitamente ou os defina como null (não undefined).

---