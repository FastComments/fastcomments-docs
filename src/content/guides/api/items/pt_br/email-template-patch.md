[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint da API permite atualizar um modelo de e-mail especificando apenas o id e os atributos a serem atualizados.

Observe que todas as mesmas validações para criar um modelo também se aplicam, por exemplo:

- O modelo deve renderizar. Isso é verificado a cada atualização.
- Não é permitido ter modelos duplicados para o mesmo domínio (caso contrário, um deles seria ignorado silenciosamente).

[inline-code-attrs-start title = 'Exemplo cURL PATCH EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta PATCH EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Incluído em caso de falha. **/
    reason?: string
    /** O modelo de e-mail atualizado. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]