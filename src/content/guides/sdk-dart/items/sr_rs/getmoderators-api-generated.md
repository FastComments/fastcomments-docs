## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Odgovor

Returns: `GetModeratorsResponse`

## Primer

[inline-code-attrs-start title = 'Primer getModerators'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Подеси ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentari ispod da postaviš prefiks (npr. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getModerators(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerators: $e\n');
}
[inline-code-end]