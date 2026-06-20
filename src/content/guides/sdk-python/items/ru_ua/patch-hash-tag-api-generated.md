## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Да |  |
| tenantId | string | query | Нет |  |

## Ответ

Возвращает: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Пример

[inline-code-attrs-start title = 'patch_hash_tag Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Примеры для каждого метода аутентификации приведены ниже, используйте пример, 
# который соответствует вашему сценарию использования.
# Настроить авторизацию по API-ключу: api_key
# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
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