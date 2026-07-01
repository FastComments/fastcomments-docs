## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |

## Ответ

Возвращает: `GetHashTagsResponse`

## Пример

[inline-code-attrs-start title = 'Пример getHashTags'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API‑ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API‑ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getHashTags(tenantId, page);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getHashTags: $e\n');
}
[inline-code-end]

---