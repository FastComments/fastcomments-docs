## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| title | string | query | No |  |

## Respuesta

Devuelve: `CreateV1PageReact`

## Ejemplo

[inline-code-attrs-start title = 'createV1PageReact Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV1PageReact(tenantId, urlId, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV1PageReact: $e\n');
}
[inline-code-end]

---