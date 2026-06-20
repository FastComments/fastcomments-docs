## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| repliesToUserId | string | query | いいえ |  |
| page | number | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## 例

[inline-code-attrs-start title = 'get_comments_for_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
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
    user_id = 'user_id_example' # str |  (オプション)
    direction = client.SortDirections() # SortDirections |  (オプション)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (オプション)
    page = 3.4 # float |  (オプション)
    includei10n = True # bool |  (オプション)
    locale = 'locale_example' # str |  (オプション)
    is_crawler = True # bool |  (オプション)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]