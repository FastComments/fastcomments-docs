## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| locale | string | query | Нет |  |

## Ответ

Возвращает: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример render_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template200_response import RenderEmailTemplate200Response
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности API-сервера.
# Ниже приведены примеры для каждого метода аутентификации; используйте пример,
# который соответствует вашему сценарию использования.

# Настроить авторизацию с помощью API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (необязательно)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]

---