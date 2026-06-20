テナントの一括ユーザー情報。userIdsが与えられると、User / SSOUserから表示情報を返します。
コメントウィジェットによって、プレゼンスイベントでちょうど出現したユーザーの情報を補完するために使用されます。
ページコンテキストなし：プライバシーは一律に適用されます（非公開プロフィールはマスクされます）。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| ids | string | query | はい | カンマ区切りの userIds. |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## 例

[inline-code-attrs-start title = 'get_users_info 例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | カンマ区切りの userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]