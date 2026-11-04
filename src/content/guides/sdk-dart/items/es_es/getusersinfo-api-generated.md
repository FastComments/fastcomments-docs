Información de usuario en lote para un tenant. Dados userIds, devuelve la información de visualización de User / SSOUser.
Usado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de manera uniforme (los perfiles privados se ocultan).

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| ids | string | query | Sí | userIds delimitados por comas. |

## Respuesta

Devuelve: `PageUsersInfoResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds delimitados por comas.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]