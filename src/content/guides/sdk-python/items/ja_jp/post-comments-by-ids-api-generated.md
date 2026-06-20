## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## 例

[inline-code-attrs-start title = 'post_comments_by_ids の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comments_by_ids_params import CommentsByIdsParams
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧については configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使用してコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    comments_by_ids_params = client.CommentsByIdsParams() # CommentsByIdsParams | 
    sso = 'sso_example' # str | （任意）

    try:
        api_response = api_instance.post_comments_by_ids(comments_by_ids_params, sso=sso)
        print("The response of ModerationApi->post_comments_by_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_comments_by_ids: %s\n" % e)
[inline-code-end]