## Parameter

| Name     | Typ    | Standort | Erforderlich | Beschreibung |
|----------|--------|----------|--------------|--------------|
| tenantId | string | query    | Ja           |  |
| id       | string | path     | Ja           |  |

## Antwort

Returns: `GetQuestionConfigResponse`

## Beispiel

[inline-code-attrs-start title = 'getQuestionConfig Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Zeile auskommentieren, um Präfix (z. B. Bearer) für API-Schlüssel festzulegen, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfig: $e\n');
}
[inline-code-end]

---