```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO‑Hilfsfunktionen sind im Paket (`package:fastcomments_dart/sso/...`) enthalten.

Der Client stellt drei API‑Klassen bereit:

- `DefaultApi` – API‑Key‑authentifizierte Methoden für die serverseitige Nutzung.
- `PublicApi` – öffentliche Methoden, die keinen API‑Schlüssel benötigen und sicher für Browser‑ und Mobil‑Clients sind.
- `ModerationApi` – ein umfangreiches Set aus Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode nimmt einen `sso`‑Parameter entgegen und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.

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