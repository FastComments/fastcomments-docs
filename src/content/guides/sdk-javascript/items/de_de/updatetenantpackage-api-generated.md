## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'updateTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_sf_001";
  const id: string = "pkg-premium-v2";
  const updateTenantPackageBody: UpdateTenantPackageBody = {
    name: "San Francisco Premium",
    enabled: true,
    customConfig: { maxComments: 500 },
    tosConfig: { required: true } // optionale Felder werden durch Anwesenheit gezeigt; andere ausgelassen
  } as UpdateTenantPackageBody;
  const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
  console.log(result);
})();
[inline-code-end]

---