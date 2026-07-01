現在オンラインのページ閲覧者: 現在ページにサブスクライブされているWebSocketセッションを持つユーザーです。  
anonCount + totalCount を返します（部屋全体の購読者数で、列挙しない匿名閲覧者も含みます）。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページURLの識別子（サーバー側でクリーンアップ済み）。 |
| afterName | string | query | No | カーソル: 前回の応答から nextAfterName を渡す。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回の応答から nextAfterUserId を渡す。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。 |

## Response

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'get_online_usersの例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# configuration.py を参照すると、サポートされているすべての設定パラメータの一覧が確認できます。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | ページURLの識別子（サーバー側でクリーンアップ済み）。
    after_name = 'after_name_example' # str | カーソル: 前回の応答から nextAfterName を渡す。 (optional)
    after_user_id = 'after_user_id_example' # str | カーソルのタイブレーカー: 前回の応答から nextAfterUserId を渡す。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。 (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]