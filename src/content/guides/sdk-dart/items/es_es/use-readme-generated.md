```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Los ayudantes SSO están incluidos en el paquete (`package:fastcomments_dart/sso/...`).

El cliente expone tres clases de API:

- `DefaultApi` - métodos autenticados con clave API para uso del lado del servidor.
- `PublicApi` - métodos públicos que no requieren clave API, seguros para navegadores y
  clientes móviles.
- `ModerationApi` - una suite extensa de APIs de moderación en tiempo real y rápidas. Cada método `ModerationApi` toma un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.

```dart
import 'package:fastcomments_dart/api.dart';

final api = PublicApi(ApiClient(basePath: 'https://fastcomments.com'));
final comments = await api.getCommentsPublic('YOUR_TENANT_ID', 'YOUR_URL_ID');
```

```dart
import 'package:fastcomments_dart/api.dart';

final publicApi = PublicApi(ApiClient(basePath: 'https://fastcomments.com'));
final feedPosts = await publicApi.getFeedPostsPublic('YOUR_TENANT_ID');
```

```dart
import 'package:fastcomments_dart/api.dart';

final moderation = ModerationApi(ApiClient(basePath: 'https://fastcomments.com'));
final result = await moderation.getApiComments(
  GetApiCommentsOptions(sso: 'SSO_TOKEN'),
);
```
```