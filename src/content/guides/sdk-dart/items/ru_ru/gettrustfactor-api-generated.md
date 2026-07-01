## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `GetUserTrustFactorResponse`

## Пример

[inline-code-attrs-start title = 'getTrustFactor Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getTrustFactor(tenantId, GetTrustFactorOptions(userId: userId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getTrustFactor: $e\n');
}
[inline-code-end]