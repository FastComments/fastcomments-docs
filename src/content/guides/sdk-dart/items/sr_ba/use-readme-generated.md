```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomagala su uključena u paket (`package:fastcomments_dart/sso/...`).

Klijent izlaže tri API klase:

- `DefaultApi` - metode autentifikovane API ključem za korištenje na serveru.
- `PublicApi` - javne metode koje ne zahtijevaju API ključ, sigurne za preglednik i mobilne klijente.
- `ModerationApi` - opsežan skup live i brzih moderacijskih API‑ja. Svaka `ModerationApi` metoda prima `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.

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