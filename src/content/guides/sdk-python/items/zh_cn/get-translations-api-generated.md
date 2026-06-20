## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | 是 |  |
| component | string | path | 是 |  |
| locale | string | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |

## 响应

返回: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## 示例

[inline-code-attrs-start title = 'get_translations 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# 设置 host 是可选的，默认值为 https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 ApiClient 实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (可选)
    use_full_translation_ids = True # bool |  (可选)

    try:
        api_response = api_instance.get_translations(namespace, component, locale=locale, use_full_translation_ids=use_full_translation_ids)
        print("The response of PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_translations: %s\n" % e)
[inline-code-end]