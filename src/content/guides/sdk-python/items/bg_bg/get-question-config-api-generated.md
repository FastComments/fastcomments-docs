## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config200_response import GetQuestionConfig200Response
from client.rest import ApiException
from pprint import pprint

# Задаването на хост е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# в съответствие с правилата за сигурност на API сървъра.
# По-долу са дадени примери за всеки метод на удостоверяване, използвайте примера, който
# удовлетворява вашия случай на използване за удостоверяване.

# Конфигуриране на удостоверяване чрез API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]