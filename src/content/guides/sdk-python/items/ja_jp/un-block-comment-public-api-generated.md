## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## レスポンス

戻り値: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/un_block_comment_public200_response.py)

## 例

[inline-code-attrs-start title = 'un_block_comment_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_block_from_comment_params import PublicBlockFromCommentParams
from client.models.un_block_comment_public200_response import UnBlockCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    public_block_from_comment_params = client.PublicBlockFromCommentParams() # PublicBlockFromCommentParams | 
    sso = 'sso_example' # str |  （オプション）

    try:
        api_response = api_instance.un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso=sso)
        print("The response of PublicApi->un_block_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_block_comment_public: %s\n" % e)
[inline-code-end]