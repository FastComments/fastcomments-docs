## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| approved | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_approved_response.py)

## 例

[inline-code-attrs-start title = 'post_set_comment_approval_status の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_approved_response import SetCommentApprovedResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスのコンテキストを開きます
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    approved = True # bool |  (省略可能)
    sso = 'sso_example' # str |  (省略可能)

    try:
        api_response = api_instance.post_set_comment_approval_status(comment_id, approved=approved, sso=sso)
        print("The response of ModerationApi->post_set_comment_approval_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_approval_status: %s\n" % e)
[inline-code-end]