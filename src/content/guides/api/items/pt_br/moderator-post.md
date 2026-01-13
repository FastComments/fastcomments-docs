[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Esta rota permite adicionar um único `Moderator`.

A criação de um `Moderator` tem as seguintes restrições:

- Um `name` e um `email` devem sempre ser fornecidos. Um `userId` é opcional.
- Os seguintes valores não podem ser fornecidos ao criar um `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Quando um `userId` é especificado, esse usuário deve existir.
- Quando um `userId` é especificado, ele deve pertencer ao mesmo `tenantId` especificado nos parâmetros de consulta.
- Dois moderadores no mesmo tenant não podem ser adicionados com o mesmo `email`.

Podemos criar um `Moderator` para um usuário do qual conhecemos apenas o e-mail:

[inline-code-attrs-start title = 'Exemplo cURL: criação de Moderador por e-mail'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ou podemos criar um `Moderator` para um usuário que pertence ao nosso tenant, para rastrear suas estatísticas de moderação:

[inline-code-attrs-start title = 'Exemplo cURL: criação de Moderador via usuário do tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Criação de Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Criação de Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    moderator?: Moderator; // Retornamos o moderador completo criado em caso de sucesso.
}
[inline-code-end]

---