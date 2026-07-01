```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomoćni alati su uključeni u paket (`package:fastcomments_dart/sso/...`).

Klijent izlaže tri API klase:

- `DefaultApi` - Metode autentifikovane API ključem za server‑side upotrebu.
- `PublicApi` - Javne metode koje ne zahtijevaju API ključ, sigurne za preglednike i mobilne klijente.
- `ModerationApi` - Opsežan skup live i brzih API‑ja za moderaciju. Svaka `ModerationApi` metoda prima `sso` parametar i može se autentifikovati putem SSO‑a ili sesijskog kolačića FastComments.com.

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