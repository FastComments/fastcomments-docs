```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Les aides SSO sont incluses dans le package (`package:fastcomments_dart/sso/...`).

Le client expose trois classes API :

- `DefaultApi` - Méthodes authentifiées par clé API pour une utilisation côté serveur.
- `PublicApi` - Méthodes publiques qui ne nécessitent aucune clé API, sécurisées pour les navigateurs et les clients mobiles.
- `ModerationApi` - une suite complète d’API de modération en direct et rapide. Chaque méthode `ModerationApi` prend un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.

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