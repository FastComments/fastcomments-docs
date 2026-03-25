## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantBody | UpdateTenantBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4821";
const id: string = "flag_7b9e";
const billingInfo: BillingInfo | undefined = undefined; // opcional, omitir para conservar la facturación actual
const updateTenantBody: UpdateTenantBody = {
  name: "Acme News Comments",
  defaultDomain: "comments.acme.com",
  ...(billingInfo ? { billingInfo } : {})
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---