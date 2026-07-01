```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO helpers are included in the package (`package:fastcomments_dart/sso/...`).

Os auxiliares SSO estão incluídos no pacote (`package:fastcomments_dart/sso/...`).

The client exposes three API classes:

O cliente expõe três classes de API:

- `DefaultApi` - API-key-authenticated methods for server-side use.  
  **métodos autenticados por chave de API para uso no lado do servidor.**

- `PublicApi` - public methods that need no API key, safe for browser and  
  mobile clients.  
  **métodos públicos que não precisam de chave de API, seguros para navegadores e  
  clientes móveis.**

- `ModerationApi` - an extensive suite of live and fast moderation APIs. Every `ModerationApi` method takes an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.  
  **uma suíte extensa de APIs de moderação ao vivo e rápidas. Cada método `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.**

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