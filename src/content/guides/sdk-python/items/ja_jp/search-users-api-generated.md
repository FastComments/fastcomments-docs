## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| usernameStartsWith | string | query | いいえ |  |
| mentionGroupIds | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| searchSection | string | query | いいえ |  |

## レスポンス

戻り値: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## 例

[inline-code-attrs-start title = 'search_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  （オプション）
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  （オプション）
    sso = 'sso_example' # str |  （オプション）
    search_section = 'search_section_example' # str |  （オプション）

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]