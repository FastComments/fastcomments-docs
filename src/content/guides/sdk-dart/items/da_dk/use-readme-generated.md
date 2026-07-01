```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO‑hjælpefunktioner er inkluderet i pakken (`package:fastcomments_dart/sso/...`).

Klienten eksponerer tre API‑klasser:

- `DefaultApi` - Metoder, der er autentificeret med API‑nøgle, til server‑side brug.
- `PublicApi` - Offentlige metoder, der ikke kræver en API‑nøgle, sikre for browser‑ og mobilklienter.
- `ModerationApi` - En omfattende suite af live og hurtige moderations‑API'er. Hver `ModerationApi`‑metode tager en `sso`‑parameter og kan autentificere via SSO eller en FastComments.com session‑cookie.

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