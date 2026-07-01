## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| locale | string | query | No |  |

## Response

Retourneert: `RenderEmailTemplateResponse`

## Example

[inline-code-attrs-start title = 'renderEmailTemplate Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de commentaar hieronder om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
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