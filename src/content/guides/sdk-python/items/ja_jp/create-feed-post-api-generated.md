## パラメータ

| 名前 | Type | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| broadcastId | string | query | いいえ |  |
| isLive | boolean | query | いいえ |  |
| doSpamCheck | boolean | query | いいえ |  |
| skipDupCheck | boolean | query | いいえ |  |

## レスポンス

返却値: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## 例

[inline-code-attrs-start title = 'create_feed_post の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは、API サーバのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例を以下に示します。自身の認証ユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーにプレフィックス（例: Bearer）を設定するには下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスのコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  （オプション）
    is_live = True # bool |  （オプション）
    do_spam_check = True # bool |  （オプション）
    skip_dup_check = True # bool |  （オプション）

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]