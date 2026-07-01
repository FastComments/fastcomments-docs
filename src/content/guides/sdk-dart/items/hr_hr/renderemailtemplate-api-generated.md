## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vraća: `RenderEmailTemplateResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer renderEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final renderEmailTemplateBody = RenderEmailTemplateBody(); // RenderEmailTemplateBody | 
final locale = locale_example; // String | 

try {
    final result = api_instance.renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->renderEmailTemplate: $e\n');
}
[inline-code-end]

---