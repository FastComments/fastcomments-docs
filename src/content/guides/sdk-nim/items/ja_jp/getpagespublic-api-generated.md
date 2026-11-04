テナントのページ一覧を取得します。FChatデスクトップクライアントがルームリストを表示するために使用します。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSOが必要なページは、リクエストユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | GetPagesPublicOptions | いいえ |  |

## レスポンス

戻り値: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]