## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_987';
const packageId: string = 'pkg_pro_2026_01';
type DeleteOptions = { force?: boolean; notify?: boolean };
const options: DeleteOptions = { force: true }; // 任意のパラメータの例
const result: FlagCommentPublic200Response = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---