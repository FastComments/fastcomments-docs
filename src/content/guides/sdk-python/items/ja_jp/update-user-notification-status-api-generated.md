## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| notificationId | string | path | はい |  |
| newStatus | string | path | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_status_response.py)

## 例

[inline-code-attrs-start title = 'update_user_notification_status の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_status_response import UpdateUserNotificationStatusResponse
from client.rest import ApiException
from pprint import pprint

# ホストの指定は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを使用してコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    new_status = 'new_status_example' # str | 
    sso = 'sso_example' # str |  (任意)

    try:
        api_response = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, sso=sso)
        print("The response of PublicApi->update_user_notification_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_status: %s\n" % e)
[inline-code-end]

---