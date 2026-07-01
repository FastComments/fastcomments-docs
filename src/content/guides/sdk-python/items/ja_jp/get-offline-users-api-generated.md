Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}`  
index from afterName forward via `$gt`, no `$skip` cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップされたもの）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから nextAfterName を渡します。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合に、名前の同点でエントリが除外されないようにするために必須です。 |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# すべてのサポートされている設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | ページ URL 識別子（サーバー側でクリーンアップされたもの）。
    after_name = 'after_name_example' # str | カーソル: 前回のレスポンスから nextAfterName を渡します。（オプション）
    after_user_id = 'after_user_id_example' # str | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合に、名前の同点でエントリが除外されないようにするために必須です。（オプション）

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]

---