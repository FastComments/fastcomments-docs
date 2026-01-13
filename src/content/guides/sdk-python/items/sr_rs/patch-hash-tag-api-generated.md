## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tag | string | path | Да |  |
| tenantId | string | query | Не |  |

## Одговор

Враћа: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_hash_tag200_response.py)

## Пример

[inline-code-attrs-start title = 'patch_hash_tag Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_hash_tag200_response import PatchHashTag200Response
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора подесити параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Пример за сваки метод аутентификације је дат испод, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење путем API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Декоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (опционално)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (опционално)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]

---