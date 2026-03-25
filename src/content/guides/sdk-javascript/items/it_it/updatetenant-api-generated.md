## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateTenantBody | UpdateTenantBody | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4821";
const id: string = "flag_7b9e";
const billingInfo: BillingInfo | undefined = undefined; // opzionale, omettere per mantenere la fatturazione corrente
const updateTenantBody: UpdateTenantBody = {
  name: "Acme News Comments",
  defaultDomain: "comments.acme.com",
  ...(billingInfo ? { billingInfo } : {})
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---