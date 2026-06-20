---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| skip | integer | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |
| contextUserId | string | query | いいえ |  |
| hashTag | string | query | いいえ |  |
| parentId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| fromDate | integer | query | いいえ |  |
| toDate | integer | query | いいえ |  |

## レスポンス

戻り値: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_comments_response.rb)

## 例

[inline-code-attrs-start title = 'get_comments の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 認証の設定
FastCommentsClient.configure do |config|
  # APIキー認証を設定: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # APIキーに接頭辞を設定するには以下の行のコメントを解除してください。例: 'Bearer'（デフォルトは nil）
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
opts = {
  page: 56, # 整数 | 
  limit: 56, # 整数 | 
  skip: 56, # 整数 | 
  as_tree: true, # ブール値 | 
  skip_children: 56, # 整数 | 
  limit_children: 56, # 整数 | 
  max_tree_depth: 56, # 整数 | 
  url_id: 'url_id_example', # 文字列 | 
  user_id: 'user_id_example', # 文字列 | 
  anon_user_id: 'anon_user_id_example', # 文字列 | 
  context_user_id: 'context_user_id_example', # 文字列 | 
  hash_tag: 'hash_tag_example', # 文字列 | 
  parent_id: 'parent_id_example', # 文字列 | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  from_date: 789, # 整数 | 
  to_date: 789 # 整数 | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]

---