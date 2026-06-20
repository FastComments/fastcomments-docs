ページ上で過去にコメントしたが現在オンラインではないユーザー。displayName でソートされています。
「Members」セクションを表示するために /users/online を使い切った後にこれを使用してください。
commenterName によるカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを afterName から $gt を使って前方へ走査します。$skip コストはありません。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページのURL識別子（サーバー側でクリーンアップされます）。 |
| afterName | string | query | いいえ | カーソル: 前回のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前が同一のエントリが失われないようこれが必要です。 |

## レスポンス

返却: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## 例

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | ページ URL の識別子（サーバー側でクリーンアップされます）。
opts = {
  after_name: 'after_name_example', # String | カーソル: 前回のレスポンスの nextAfterName を渡します。
  after_user_id: 'after_user_id_example' # String | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前が同一のエントリが失われないようこれが必要です。
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]