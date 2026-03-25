## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'replaceTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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