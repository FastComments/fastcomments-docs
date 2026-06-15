## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Oui |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-114";
const id: string = "pkg-enterprise-2026-06";
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: "EnterpriseModeration",
  version: "2.4.7",
  enabled: true,
  apiStatus: { state: "active", lastUpdated: "2026-06-10T12:00:00Z" },
  customConfigParameters: { maxCommentLength: 1200, allowImages: true }, // paramètres optionnels inclus
  voteStyle: { style: "updown" }
};
const result: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---