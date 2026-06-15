## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-123';
const id: string = 'template-789';
const locale: string | undefined = 'en-US';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  subject: 'Welcome to Acme — Get started',
  bodyHtml: '<p>Hi \{{firstName}}, welcome to Acme. Start by visiting your dashboard.</p>',
  fromName: 'Acme Support',
  fromEmail: 'support@acme.com',
  enabled: true,
  ...(locale ? { locale } : {})
};
const result: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]