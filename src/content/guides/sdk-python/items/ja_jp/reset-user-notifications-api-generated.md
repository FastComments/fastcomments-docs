## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| afterId | string | query | いいえ |  |
| afterCreatedAt | integer | query | いいえ |  |
| unreadOnly | boolean | query | いいえ |  |
| dmOnly | boolean | query | いいえ |  |
| noDm | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## 例

[inline-code-attrs-start title = 'reset_user_notifications の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスとともにコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  （任意）
    after_created_at = 56 # int |  （任意）
    unread_only = True # bool |  （任意）
    dm_only = True # bool |  （任意）
    no_dm = True # bool |  （任意）
    sso = 'sso_example' # str |  （任意）

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso)
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]