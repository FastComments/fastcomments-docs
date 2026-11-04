現在オンラインではない、そのページの過去のコメント投稿者。displayName でソートされます。
「Members」セクションを表示する際は、まず /users/online を使い尽くした後にこれを使用してください。
commenterName によるカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName から $gt で前方に走査し、$skip のコストは発生しません。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL 識別子(サーバー側でクリーンアップされます)。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は必須です。同姓同名のエントリが欠落しないようにするためです。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 例

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | ページ URL 識別子(サーバー側でクリーンアップされます)。
    after_name = 'after_name_example' # str | カーソル: 前のレスポンスの nextAfterName を渡します。 (optional)
    after_user_id = 'after_user_id_example' # str | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は必須です。同姓同名のエントリが欠落しないようにするためです。 (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]