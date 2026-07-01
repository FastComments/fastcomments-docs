## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteEmailTemplate Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // Παράδειγμα πρόσβασης σε προαιρετική ιδιότητα από την απάντηση
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]