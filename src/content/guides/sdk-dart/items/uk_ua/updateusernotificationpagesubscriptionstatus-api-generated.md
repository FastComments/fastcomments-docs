Увімкніть або вимкніть сповіщення для сторінки. Коли користувачі підписані на сторінку, сповіщення створюються
для нових кореневих коментарів, а також

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Так |  |
| url | string | query | Так |  |
| pageTitle | string | query | Так |  |
| subscribedOrUnsubscribed | string | path | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: `UpdateUserNotificationPageSubscriptionStatusResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад updateUserNotificationPageSubscriptionStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final url = url_example; // String | 
final pageTitle = pageTitle_example; // String | 
final subscribedOrUnsubscribed = subscribedOrUnsubscribed_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: $e\n');
}
[inline-code-end]