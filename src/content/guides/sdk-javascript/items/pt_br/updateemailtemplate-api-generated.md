## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Sim |  |

## Resposta

Retorna: [`UpdateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateEmailTemplateResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "email_tpl_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "Comment reply notification",
    htmlContent: "<p>Someone replied to your comment.</p>",
    plainTextContent: "Someone replied to your comment.",
    // exemplo de campo opcional
    isActive: true,
  };

  const result: UpdateEmailTemplateResponse = await updateEmailTemplate(
    tenantId,
    templateId,
    updateBody
  );

  console.log(result);
})();
[inline-code-end]