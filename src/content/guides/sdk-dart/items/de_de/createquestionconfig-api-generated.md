## Parameter

| Name     | Typ    | Ort    | Erforderlich | Beschreibung |
|----------|--------|--------|--------------|--------------|
| tenantId | string | query  | Ja           |  |

## Antwort

Returns: `CreateQuestionConfigResponse`

## Beispiel

[inline-code-attrs-start title = 'createQuestionConfig Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Autorisierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie die Auskommentierung unten, um das Präfix (z.B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionConfigBody = CreateQuestionConfigBody(); // CreateQuestionConfigBody | 

try {
    final result = api_instance.createQuestionConfig(tenantId, createQuestionConfigBody);
    print(result);
} catch (e) {
    // Ausnahme beim Aufruf von DefaultApi->createQuestionConfig: $e
    print('Exception when calling DefaultApi->createQuestionConfig: $e\n');
}
[inline-code-end]