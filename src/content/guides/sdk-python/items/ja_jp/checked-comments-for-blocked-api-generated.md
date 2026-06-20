## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | コメントIDのカンマ区切りリスト。 |
| sso | string | query | No |  |

## レスポンス

戻り値: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/check_blocked_comments_response.py)

## 例

[inline-code-attrs-start title = 'checked_comments_for_blocked の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.check_blocked_comments_response import CheckBlockedCommentsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py に、サポートされているすべての設定パラメータの一覧があります。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | コメントIDのカンマ区切りリスト。
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]