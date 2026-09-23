## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Sim |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "New Comment Notification",
    // htmlContent é opcional e omitido
    status: { code: 200, message: "Active" } // APIStatus
  };

  const response: APIEmptyResponse = await updateEmailTemplate(tenantId, templateId, updateBody);
  console.log(response);
}
[inline-code-end]

---