```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO-hulpmiddelen zijn inbegrepen in het pakket (`package:fastcomments_dart/sso/...`).

De client exposeert drie API-klassen:

- `DefaultApi` - API-sleutel‑geauthenticeerde methoden voor server‑side gebruik.
- `PublicApi` - openbare methoden die geen API‑sleutel nodig hebben, veilig voor browsers en
  mobiele clients.
- `ModerationApi` - een uitgebreide suite van live en snelle moderatie‑API's. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com sessie‑cookie.

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