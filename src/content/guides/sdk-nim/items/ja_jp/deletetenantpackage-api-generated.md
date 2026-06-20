## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

返却値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'deleteTenantPackage の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "package-987")
if response.isSome:
  let emptyResp = response.get()
  echo emptyResp
else:
  echo "Failed to delete tenant package"
[inline-code-end]

---