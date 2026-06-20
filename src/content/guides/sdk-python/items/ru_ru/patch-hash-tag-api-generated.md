## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Да |  |
| tenantId | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Пример

[inline-code-attrs-start title = 'Пример patch_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию — https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Ниже приведены примеры для каждого метода аутентификации, используйте пример, который
# соответствует вашему сценарию использования.

# Настройка авторизации с помощью API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если требуется
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (необязательно)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (необязательно)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]