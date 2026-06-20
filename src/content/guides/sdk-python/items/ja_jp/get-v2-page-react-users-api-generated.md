## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |

## レスポンス

戻り値: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_react_users_response.py)

## 例

[inline-code-attrs-start title = 'get_v2_page_react_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_react_users_response import GetV2PageReactUsersResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
        print("The response of PublicApi->get_v2_page_react_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_react_users: %s\n" % e)
[inline-code-end]