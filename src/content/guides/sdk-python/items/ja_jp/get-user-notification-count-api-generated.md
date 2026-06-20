## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notification_count_response.py)

## 例

[inline-code-attrs-start title = 'get_user_notification_count の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notification_count_response import GetUserNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# ホストの指定は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成する
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.get_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->get_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notification_count: %s\n" % e)
[inline-code-end]