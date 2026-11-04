```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Os helpers de SSO estão incluídos no pacote (`package:fastcomments_dart/sso/...`).

O cliente expõe três classes de API:

- `DefaultApi` - métodos autenticados por chave de API para uso no lado do servidor.
- `PublicApi` - métodos públicos que não precisam de chave de API, seguros para clientes de navegador e mobile.
- `ModerationApi` - um conjunto extenso de APIs de moderação ao vivo e rápida. Todo método de `ModerationApi` recebe um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.

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