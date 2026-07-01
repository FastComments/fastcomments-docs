## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## Απάντηση

Επιστρέφει: `APIEmptyResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'updateTenantUser Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantUserBody = UpdateTenantUserBody(); // UpdateTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantUser: $e\n');
}
[inline-code-end]