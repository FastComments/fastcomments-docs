## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Yes |  |

## Απάντηση

Επιστρέφει: `CreateEmailTemplateResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createEmailTemplateBody = CreateEmailTemplateBody(); // CreateEmailTemplateBody | 

try {
    final result = api_instance.createEmailTemplate(tenantId, createEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createEmailTemplate: $e\n');
}
[inline-code-end]