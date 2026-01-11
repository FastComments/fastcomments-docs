## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_id_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_sso_user_by_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_id_api_response import GetSSOUserByIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиентът трябва да конфигурира параметрите за автентикация и авторизация
# в съответствие с политиката за сигурност на API сървъра.
# По-долу са предоставени примери за всеки метод за удостоверяване; използвайте примера, който
# отговаря на вашия случай на използване.
 
# Конфигурирайте упълномощаването с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Разкоментирайте по-долу, за да зададете префикс (e.g. Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_id(tenant_id, id)
        print("The response of DefaultApi->get_sso_user_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_id: %s\n" % e)
[inline-code-end]