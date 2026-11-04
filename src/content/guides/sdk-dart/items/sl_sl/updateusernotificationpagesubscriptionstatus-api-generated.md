Omogoči ali onemogoči obvestila za stran. Ko so uporabniki naročeni na stran, se obvestila ustvarijo
za nove korenske komentarje, pa tudi

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| url | string | query | Da |  |
| pageTitle | string | query | Da |  |
| subscribedOrUnsubscribed | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `UpdateUserNotificationPageSubscriptionStatusResponse`

## Primer

[inline-code-attrs-start title = 'Primer updateUserNotificationPageSubscriptionStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
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