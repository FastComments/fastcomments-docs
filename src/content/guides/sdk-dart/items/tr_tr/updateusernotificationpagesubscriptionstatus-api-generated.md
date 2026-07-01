Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, yeni kök yorumlar için bildirimler oluşturulur ve ayrıca

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Evet |  |
| url | string | query | Evet |  |
| pageTitle | string | query | Evet |  |
| subscribedOrUnsubscribed | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: `UpdateUserNotificationPageSubscriptionStatusResponse`

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---