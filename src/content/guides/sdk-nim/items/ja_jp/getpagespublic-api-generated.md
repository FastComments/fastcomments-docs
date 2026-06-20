テナントのページを一覧表示します。FChat デスクトップクライアントがルームリストを生成するために使用します。
各ページの解決済みカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、リクエスト元ユーザーのグループアクセスに対してフィルタされます。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| cursor | string | いいえ |  |
| limit | int | いいえ |  |
| q | string | いいえ |  |
| sortBy | PagesSortBy | いいえ |  |
| hasComments | bool | いいえ |  |

## レスポンス

戻り値: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---