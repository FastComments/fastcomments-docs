## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|------------|-------------|
| tenantId | string | path | Sí |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `CreateFeedPostResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createFeedPostPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final createFeedPostParams = CreateFeedPostParams(); // CreateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.createFeedPostPublic(tenantId, createFeedPostParams, CreateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createFeedPostPublic: $e\n');
}
[inline-code-end]