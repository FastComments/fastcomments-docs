```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO pomoćnici su uključeni u paket (`package:fastcomments_dart/sso/...`).

Klijent izlaže tri API klase:

- `DefaultApi` - Metode autentifikovane API ključem za upotrebu na serveru.
- `PublicApi` - javne metode koje ne zahtevaju API ključ, bezbedne za pregledače i mobilne klijente.
- `ModerationApi` - opsežan paket live i brzih moderacionih API‑ja. Svaka metoda `ModerationApi` prima `sso` parametar i može se autentifikovati preko SSO ili sesijskog kolačića FastComments.com.

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