## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f3c2a";
const id: string = "pkg_4f8b21";
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  packageName: "Premium Moderation Pack",
  enabled: true,
  apiStatus: { mode: "active" } as APIStatus,
  customConfigParameters: { maxFlagsBeforeReview: 5 } as CustomConfigParameters,
  voteStyle: "thumbs" as VoteStyle,
  tosConfig: { requireAcceptance: true } as TOSConfig
};
const result: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---