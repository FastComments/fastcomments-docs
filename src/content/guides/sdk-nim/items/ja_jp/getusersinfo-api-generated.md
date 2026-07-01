テナントの大量ユーザー情報。userIds が与えられると、User / SSOUser から表示情報を返します。コメントウィジェットがプレゼンスイベントで新たに現れたユーザーを豊かにするために使用されます。ページコンテキストがないため、プライバシーは一律に適用されます（非公開プロファイルはマスクされます）。

## パラメータ

| 名前 | 型 | 必要 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| ids | string | いいえ |  |

## レスポンス

返却: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]