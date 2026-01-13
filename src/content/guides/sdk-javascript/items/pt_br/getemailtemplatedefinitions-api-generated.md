---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |

## Resposta

Retorna: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-78f2';
const emailTemplates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId);
[inline-code-end]

---