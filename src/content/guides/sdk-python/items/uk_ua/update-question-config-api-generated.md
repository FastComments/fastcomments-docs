## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад update_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_question_config_body import UpdateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Вказувати хост необов'язково; за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри аутентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте той
# який відповідає вашому випадку використання.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст із екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_config_body = client.UpdateQuestionConfigBody() # UpdateQuestionConfigBody | 

    try:
        api_response = api_instance.update_question_config(tenant_id, id, update_question_config_body)
        print("The response of DefaultApi->update_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_config: %s\n" % e)
[inline-code-end]