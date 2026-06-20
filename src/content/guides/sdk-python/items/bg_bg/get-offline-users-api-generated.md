Предишни коментирали на страницата, които НЕ са в момента онлайн. Подредени по displayName.
Използвайте това след като изчерпате /users/online, за да визуализирате секция "Членове".
Курсорна пагинация по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (обработен от страна на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не се пропускат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на класа API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор на URL на страницата (обработен от страна на сървъра).
    after_name = 'after_name_example' # str | Курсор: предайте nextAfterName от предишния отговор. (по избор)
    after_user_id = 'after_user_id_example' # str | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададен, за да не се пропускат записи при еднакви имена. (по избор)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]