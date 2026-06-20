---
現在ページにオンラインの閲覧者: 現在そのページに WebSocket セッションでサブスクライブしている人々。
anonCount + totalCount を返します（匿名閲覧者を列挙しない場合でも含めた、ルーム全体の購読者数）。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページの URL 識別子（サーバー側で正規化されます）。 |
| afterName | string | query | No | カーソル: 前のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前の重複でエントリが落ちないように必須です。 |

## レスポンス

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## 例

[inline-code-attrs-start title = 'get_online_users の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | ページの URL 識別子（サーバー側で正規化されます）。
opts = {
  after_name: 'after_name_example', # String | カーソル: 前のレスポンスの nextAfterName を渡します。
  after_user_id: 'after_user_id_example' # String | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前の重複でエントリが落ちないように必須です。
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---