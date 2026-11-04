[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Este endpoint de API fornece a capacidade de criar modelos de e-mail.

Observações:

- Você não pode ter vários modelos com o mesmo `emailTemplateId` no mesmo domínio.
- Mas você pode ter um modelo curinga (`domain` = `*` e um modelo específico de domínio para o mesmo `emailTemplateId`).
- Especificar `domain` só é relevante se você tiver domínios diferentes, ou quiser usar modelos específicos para testes (`domain` definido como `localhost` etc).
- Se você especificar `domain`, ele deve corresponder a um `DomainConfig`. Em caso de erro, uma lista de domínios válidos é fornecida.
- A sintaxe do modelo é EJS e é renderizada com um tempo limite de 500ms. O P99 de renderização é <5ms, então se você atingir 500ms algo está errado.
- **Seu modelo deve renderizar com o `testData` fornecido** para ser salvo. Erros de renderização são agregados e reportados no painel (em breve disponível via API).

Os dados mínimos necessários para adicionar um modelo são os seguintes:

[inline-code-attrs-start title = 'Exemplo mínimo de cURL POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Você pode querer ter modelos por site, nesse caso você define `domain`:

[inline-code-attrs-start title = 'Exemplo de cURL POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Incluído em caso de falha. **/
    reason?: string
    /** O modelo criado. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]