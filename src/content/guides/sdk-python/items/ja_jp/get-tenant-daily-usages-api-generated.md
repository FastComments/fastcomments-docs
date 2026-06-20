## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| yearNumber | number | query | いいえ |  |
| monthNumber | number | query | いいえ |  |
| dayNumber | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## 例

[inline-code-attrs-start title = 'get_tenant_daily_usages の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。

# クライアントは認証および認可のパラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従ってください。
# 各認証方法の例を以下に示します。例の中から
# ご自身のユースケースに合うものを使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、以下のコメントアウトを外してAPIキーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスを使ってコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (optional)
    month_number = 3.4 # float |  (optional)
    day_number = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]