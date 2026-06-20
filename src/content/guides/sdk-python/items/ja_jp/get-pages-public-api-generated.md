テナントのページ一覧を取得します。FChat デスクトップクライアントがルームリストを表示するために使用します。各ページに対する解決済みカスタム設定で `enableFChat` が true である必要があります。SSO が必要なページは、要求元ユーザーのグループアクセスに基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 以前のリクエストから返される不透明なページネーションカーソル。`nextCursor` として返されます。`sortBy` と紐づきます。 |
| limit | integer | query | いいえ | 1..200、デフォルト 50 |
| q | string | query | いいえ | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | 並び順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多いものが先）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | いいえ | true の場合、少なくとも1件のコメントがあるページのみを返します。 |

## レスポンス

返却値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 例

[inline-code-attrs-start title = 'get_pages_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# すべてのサポートされている構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを用いてコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 以前のリクエストから返される不透明なページネーションカーソル。`nextCursor` として返されます。`sortBy` と紐づきます。 (optional)
    limit = 56 # int | 1..200、デフォルト 50 (optional)
    q = 'q_example' # str | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | 並び順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多いものが先）、または `title`（アルファベット順）。 (optional)
    has_comments = True # bool | true の場合、少なくとも1件のコメントがあるページのみを返します。 (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]