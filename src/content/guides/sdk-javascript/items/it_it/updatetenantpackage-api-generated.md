## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_sf_001";
  const id: string = "pkg-premium-v2";
  const updateTenantPackageBody: UpdateTenantPackageBody = {
    name: "San Francisco Premium",
    enabled: true,
    customConfig: { maxComments: 500 },
    tosConfig: { required: true } // campi opzionali dimostrati dalla presenza; gli altri omessi
  } as UpdateTenantPackageBody;
  const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
  console.log(result);
})();
[inline-code-end]

---