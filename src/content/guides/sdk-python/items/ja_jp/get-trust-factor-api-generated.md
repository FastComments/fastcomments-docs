## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

戻り値: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## 例

[inline-code-attrs-start title = 'get_trust_factor の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを用いてコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (任意)
    sso = 'sso_example' # str |  (任意)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]