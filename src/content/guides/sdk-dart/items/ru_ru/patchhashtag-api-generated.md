## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| tag | string | path | Да |  |

## Ответ

Возвращает: `UpdateHashTagResponse`

## Пример

[inline-code-attrs-start title = 'Пример patchHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию с помощью API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final updateHashTagBody = UpdateHashTagBody(); // UpdateHashTagBody | 

try {
    final result = api_instance.patchHashTag(tenantId, tag, updateHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchHashTag: $e\n');
}
[inline-code-end]

---