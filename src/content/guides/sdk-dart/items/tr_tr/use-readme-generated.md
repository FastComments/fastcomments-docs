```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO yardımcıları paket içinde dahil edilmiştir (`package:fastcomments_dart/sso/...`).

İstemci üç API sınıfı sunar:

- `DefaultApi` - Sunucu tarafı kullanım için API anahtarıyla kimliği doğrulanmış yöntemler.
- `PublicApi` - API anahtarı gerektirmeyen genel yöntemler, tarayıcı ve mobil istemciler için güvenlidir.
- `ModerationApi` - Canlı ve hızlı moderasyon API'lerinin kapsamlı bir paketi. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çereziyle kimlik doğrulaması yapabilir.

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