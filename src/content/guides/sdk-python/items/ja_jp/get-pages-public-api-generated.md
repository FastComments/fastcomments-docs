---
テナントのページを一覧取得します。FChat デスクトップクライアントがルームリストを生成する際に使用されます。  
`enableFChat` が各ページの解決されたカスタム設定で true であることが必要です。  
SSO が必要なページは、リクエストしたユーザーのグループアクセスに基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソルです。同じ `sortBy` に紐付けられます。 |
| limit | integer | query | No | 1..200、デフォルトは 50 |
| q | string | query | No | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタです。 |
| sortBy | string | query | No | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | No | true の場合、少なくとも1件のコメントがあるページのみ返します。 |

## レスポンス

返却: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 例

[inline-code-attrs-start title = 'get_pages_public 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py を参照すると、サポートされているすべての設定パラメータの一覧が確認できます。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスとともにコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 前回のリクエストで `nextCursor` として返された不透明なページネーションカーソルです。同じ `sortBy` に紐付けられます。(オプション)
    limit = 56 # int | 1..200、デフォルト 50 (オプション)
    q = 'q_example' # str | オプションの大文字小文字を区別しないタイトルプレフィックスフィルタです。(オプション)
    sort_by = client.PagesSortBy() # PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が最初）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。(オプション)
    has_comments = True # bool | true の場合、少なくとも1件のコメントがあるページのみ返します。(オプション)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---