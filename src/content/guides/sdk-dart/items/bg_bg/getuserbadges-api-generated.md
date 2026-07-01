## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Отговор

Връща: `APIGetUserBadgesResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadges'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте разрешение за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментарирайте по-долу, за да настроите префикс (напр. Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final badgeId = badgeId_example; // String | 
final type = 1.2; // double | 
final displayedOnComments = true; // bool | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadges(tenantId, GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadges: $e\n');
}
[inline-code-end]