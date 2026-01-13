## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример update_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_question_config_body import UpdateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су наведени примери за сваки метод аутентификације, користите пример који
# задовољава ваш случај коришћења аутентификације.

# Конфигуришите авторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Окоментаришите доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
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

---