## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_5f3a9c2b';
  const templates: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const skip: number = 20;
  const pagedTemplates: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
  console.log(templates, pagedTemplates);
}
main();
[inline-code-end]

---