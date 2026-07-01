## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Ответ

Возвращает: `GetTenantDailyUsagesResponse`

## Пример

[inline-code-attrs-start title = 'Пример getTenantDailyUsages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final yearNumber = 1.2; // double | 
final monthNumber = 1.2; // double | 
final dayNumber = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantDailyUsages(tenantId, GetTenantDailyUsagesOptions(yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantDailyUsages: $e\n');
}
[inline-code-end]