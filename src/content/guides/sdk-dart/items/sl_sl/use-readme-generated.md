```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomožniki so vključeni v paket (`package:fastcomments_dart/sso/...`).

Odjemalec razkriva tri API razrede:

- `DefaultApi` - metode, avtenticirane z API-ključem, za uporabo na strežniški strani.
- `PublicApi` - javne metode, ki ne potrebujejo API ključa, varne za brskalnik in
  mobilne odjemalce.
- `ModerationApi` - obsežen paket žive in hitre moderacijske API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.

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