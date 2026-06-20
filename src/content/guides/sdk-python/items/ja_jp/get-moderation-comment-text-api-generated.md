## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| sso | string | query | いいえ |  |

## Response

戻り値: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text_response.py)

## Example

[inline-code-attrs-start title = 'get_moderation_comment_text の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text_response import GetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを使用してコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (オプション)

    try:
        api_response = api_instance.get_moderation_comment_text(comment_id, sso=sso)
        print("The response of ModerationApi->get_moderation_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment_text: %s\n" % e)
[inline-code-end]

---