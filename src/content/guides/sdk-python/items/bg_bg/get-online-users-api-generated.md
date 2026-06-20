В момента онлайн зрителите на страница: хора, чиито websocket сесии са абонирани за страницата в момента.
Връща anonCount + totalCount (абонати в рамките на стаята, включително анонимни зрители, които не изброяваме).

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (изчистен на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за решаване на равенство: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададено, за да се предотврати пропускане на записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хост е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор на URL на страницата (изчистен на сървъра).
    after_name = 'after_name_example' # str | Курсор: предайте nextAfterName от предишния отговор. (по избор)
    after_user_id = 'after_user_id_example' # str | Курсор за решаване на равенство: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададено, за да се предотврати пропускане на записи при еднакви имена. (по избор)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]