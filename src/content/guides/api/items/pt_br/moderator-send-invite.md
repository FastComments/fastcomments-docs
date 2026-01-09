[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Esta rota fornece a capacidade de convidar um único `Moderator`.

As seguintes restrições existem para enviar um email de convite a um `Moderator`:
- O `Moderator` deve já existir.
- O `fromName` não pode ter mais de `100 characters`.

**Observações:**
- Se um usuário com o e-mail fornecido já existir, ele será convidado a moderar os comentários do seu tenant.
- Se um usuário com o e-mail fornecido **não existir**, o link de convite o guiará pelo processo de criação da conta.
- O convite expirará após `30 days`.

Podemos criar um `Moderator` para um usuário do qual só sabemos o e-mail:

[inline-code-attrs-start title = 'Exemplo de cURL de convite de Moderador'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Isso enviará um e-mail como `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Estrutura da Requisição de Convite de Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** O e-mail enviado ao usuário aparecerá como se tivesse sido enviado por este nome. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Convite de Moderador'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]