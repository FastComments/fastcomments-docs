## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | 是 |  |
| component | string | path | 是 |  |
| locale | string | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |

## 回應

回傳: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## 範例

[inline-code-attrs-start title = 'get_translations 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (可選)
    use_full_translation_ids = True # bool |  (可選)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]