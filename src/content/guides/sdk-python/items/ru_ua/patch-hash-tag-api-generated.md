## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| tag | string | path | Так |  |

## Відповідь

Returns: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Приклад

[inline-code-attrs-start title = 'patch_hash_tag Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням використовується https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки API сервера.
# Приклади для кожного методу автентифікації наведені нижче, використайте приклад, який
# задовольняє ваш випадок використання автентифікації.
# Налаштуйте авторизацію за API ключем: api_key
# Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'
# Відкрийте контекст з екземпляром API клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр API класу
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (необов'язковий)

    try:
        api_response = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]