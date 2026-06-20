---
テナントの大量ユーザー情報。userIds を指定すると、User / SSOUser から表示用情報を返します。
コメントウィジェットが、プレゼンスイベントでちょうど出現したユーザーを補完するために使用します。
ページコンテキストなし：プライバシーは一律に適用されます（非公開プロファイルはマスクされます）。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| ids | string | いいえ |  |

## レスポンス

戻り値: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---