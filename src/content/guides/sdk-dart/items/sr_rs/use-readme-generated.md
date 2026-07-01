```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomoćnici su uključeni u paket (`package:fastcomments_dart/sso/...`).

Klijent izlaže tri API klase:

- `DefaultApi` - metode autentifikovane API ključem za server‑side upotrebu.
- `PublicApi` - javne metode koje ne zahtevaju API ključ, sigurne za pretraživače i mobilne klijente.
- `ModerationApi` - opsežan skup API‑ja za moderaciju u realnom vremenu i brzu moderaciju. Svaka `ModerationApi` metoda prima `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.

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