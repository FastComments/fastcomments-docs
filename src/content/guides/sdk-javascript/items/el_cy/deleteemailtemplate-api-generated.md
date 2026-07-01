## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Απόκριση

Επιστρέφει: [`DeleteEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteEmailTemplateResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_abcde";

  const response: DeleteEmailTemplateResponse = await deleteEmailTemplate(tenantId, templateId);

  // Παράδειγμα πρόσβασης σε προαιρετική ιδιότητα από την απόκριση
  const statusCode: number | undefined = response.status?.code;
}();
[inline-code-end]