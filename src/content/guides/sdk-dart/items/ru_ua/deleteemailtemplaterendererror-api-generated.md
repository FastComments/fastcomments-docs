## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| errorId | string | path | Yes |  |

## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример deleteEmailTemplateRenderError'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final errorId = errorId_example; // String | 

try {
    final result = api_instance.deleteEmailTemplateRenderError(tenantId, id, errorId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplateRenderError: $e\n');
}
[inline-code-end]