## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vrne: `RenderEmailTemplateResponse`

## Primer

[inline-code-attrs-start title = 'renderEmailTemplate Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
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