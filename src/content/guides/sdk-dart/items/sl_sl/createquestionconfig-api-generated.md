## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: `CreateQuestionConfigResponse`

## Primer

[inline-code-attrs-start title = 'Primer createQuestionConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte overitev API ključev: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionConfigBody = CreateQuestionConfigBody(); // CreateQuestionConfigBody | 

try {
    final result = api_instance.createQuestionConfig(tenantId, createQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionConfig: $e\n');
}
[inline-code-end]