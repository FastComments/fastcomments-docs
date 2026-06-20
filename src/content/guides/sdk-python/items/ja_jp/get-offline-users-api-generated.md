ページ上で過去にコメントした、現在オンラインではないユーザー。displayName でソートされます。
「Members」セクションを表示するために、/users/online を使い切った後にこれを使用してください。
commenterName に対するカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName から $gt を使って前方へ走査します。$skip コストはかかりません。

## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページのURL識別子（サーバー側でクリーンアップされます）。 |
| afterName | string | query | No | カーソル: 前のレスポンスから nextAfterName を渡してください。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前のレスポンスから nextAfterUserId を渡してください。afterName が設定されている場合は、名前の同一性によってエントリが除外されないようにこれは必須です。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 例

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメーターの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを用いるコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | ページのURL識別子（サーバー側でクリーンアップされます）。
    after_name = 'after_name_example' # str | カーソル: 前のレスポンスから nextAfterName を渡してください。 (任意)
    after_user_id = 'after_user_id_example' # str | カーソルのタイブレーカー: 前のレスポンスから nextAfterUserId を渡してください。afterName が設定されている場合、名前の同一性によりエントリが除外されないようにこれは必須です。 (任意)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]