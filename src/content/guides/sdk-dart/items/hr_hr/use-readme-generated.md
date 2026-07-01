```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomoćnici su uključeni u paket (`package:fastcomments_dart/sso/...`).

Klijent izlaže tri API klase:

- `DefaultApi` – metode autentificirane API ključem za upotrebu na poslužitelju.
- `PublicApi` – javne metode koje ne zahtijevaju API ključ, sigurne za preglednike i mobilne klijente.
- `ModerationApi` – opsežan skup API-ja za moderaciju u stvarnom vremenu i brzu moderaciju. Svaka metoda `ModerationApi` prima parametar `sso` i može se autentificirati putem SSO-a ili kolačića sesije FastComments.com.

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