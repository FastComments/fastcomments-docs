## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| updateComments | string | query | Не |  |

## Одговор

Returns: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'replaceTenantUser Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// откоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantUserBody = ReplaceTenantUserBody(); // ReplaceTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantUser: $e\n');
}
[inline-code-end]