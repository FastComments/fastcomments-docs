## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

返却: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## 例

[inline-code-attrs-start title = 'post_vote 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py を参照すると、サポートされているすべての構成パラメータの一覧が確認できます
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クラスのインスタンスを作成する
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str |  (optional)
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_vote(tenant_id, comment_id, PostVoteOptions(direction=direction, broadcast_id=broadcast_id, sso=sso))
        print("ModerationApi->post_vote のレスポンス:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->post_vote の呼び出し中に例外が発生しました: %s\n" % e)
[inline-code-end]