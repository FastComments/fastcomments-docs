[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a habilidade de atualizar um `Moderator` por `id`.

A atualização de um `Moderator` tem as seguintes restrições:

- Os seguintes valores não podem ser fornecidos ao atualizar um `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Quando um `userId` for especificado, esse usuário deve existir.
- Quando um `userId` for especificado, esse usuário deve pertencer ao mesmo `tenantId` especificado nos parâmetros de consulta.
- Dois moderadores no mesmo tenant não podem ser adicionados com o mesmo `email`.
- Você não pode alterar o `tenantId` associado a um `Moderator`.

[inline-code-attrs-start title = 'Exemplo cURL PATCH de Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição PATCH de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta PATCH de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]