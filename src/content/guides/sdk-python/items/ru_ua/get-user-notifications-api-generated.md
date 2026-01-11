## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| pageSize | integer | query | Нет |  |
| afterId | string | query | Нет |  |
| includeContext | boolean | query | Нет |  |
| afterCreatedAt | integer | query | Нет |  |
| unreadOnly | boolean | query | Нет |  |
| dmOnly | boolean | query | Нет |  |
| noDm | boolean | query | Нет |  |
| includeTranslations | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notifications200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notifications200_response import GetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Указание host не обязательно, по умолчанию используется https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войти в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создать экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page_size = 56 # int |  (необязательно)
    after_id = 'after_id_example' # str |  (необязательно)
    include_context = True # bool |  (необязательно)
    after_created_at = 56 # int |  (необязательно)
    unread_only = True # bool |  (необязательно)
    dm_only = True # bool |  (необязательно)
    no_dm = True # bool |  (необязательно)
    include_translations = True # bool |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]