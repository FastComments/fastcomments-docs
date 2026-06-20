現在オンラインのページ閲覧者: 現在そのページを購読している WebSocket セッションを持つ人々。anonCount + totalCount を返します（ルーム全体の購読者数。列挙しない匿名閲覧者を含む）。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページの URL 識別子（サーバー側でクリーン処理されています）。 |
| afterName | string | query | No | カーソル: 前のレスポンスの nextAfterName を渡してください。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されているときに必要です。名前が同じ場合にエントリが落ちるのを防ぎます。 |

## Response

返却値: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'get_online_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、既定値は https://fastcomments.com です
# サポートされている全ての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | ページの URL 識別子（サーバー側でクリーン処理されています）。
    after_name = 'after_name_example' # str | カーソル: 前のレスポンスの nextAfterName を渡してください。 (optional)
    after_user_id = 'after_user_id_example' # str | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されているときに必要です。名前が同じ場合にエントリが落ちるのを防ぎます。 (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]