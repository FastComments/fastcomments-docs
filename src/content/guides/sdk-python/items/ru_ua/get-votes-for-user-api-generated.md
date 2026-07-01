## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Відповідь

Повертає: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetVotesForUserOptions
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов’язковим і за замовчуванням встановлено https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу аутентифікації наведені нижче, використайте приклад,
# який відповідає вашому випадку використання.

# Налаштувати авторизацію за API‑ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API‑ключа, за потреби
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрити контекст з екземпляром API‑клієнта
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов’язково)
    anon_user_id = 'anon_user_id_example' # str |  (необов’язково)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, GetVotesForUserOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]