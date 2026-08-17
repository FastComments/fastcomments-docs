Past commenters on the page who are NOT currently online. Sorted by displayName.  
ページ上の過去のコメント投稿者で、現在オンラインではないもの。displayName でソートされます。

Use this after exhausting /users/online to render a "Members" section.  
`/users/online` をすべて取得した後に、"Members" セクションを表示するために使用します。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName に対するカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを afterName 以降に $gt で進め、$skip コストはかかりません。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

返却: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]