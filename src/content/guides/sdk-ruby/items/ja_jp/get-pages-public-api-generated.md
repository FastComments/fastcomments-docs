テナントのページを一覧表示します。FChat デスクトップクライアントがルームリストを生成するために使用します。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、リクエスト元ユーザーのグループアクセスに基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 前のリクエストから返された不透明なページネーションカーソル（`nextCursor`）。同じ `sortBy` に紐づきます。 |
| limit | integer | query | いいえ | 1..200、デフォルト 50 |
| q | string | query | いいえ | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | いいえ | true の場合、コメントが1件以上あるページのみを返します。 |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## 例

[inline-code-attrs-start title = 'get_pages_public の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | 前のリクエストから返された不透明なページネーションカーソル（`nextCursor`）。同じ `sortBy` に紐づきます。
  limit: 56, # Integer | 1..200、デフォルト 50
  q: 'q_example', # String | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。
  has_comments: true # Boolean | true の場合、コメントが1件以上あるページのみを返します。
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---