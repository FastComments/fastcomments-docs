```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Les assistants SSO sont inclus dans le package (`package:fastcomments_dart/sso/...`).

Le client expose trois classes API :

- `DefaultApi` - Méthodes authentifiées par clé API destinées à une utilisation côté serveur.
- `PublicApi` - Méthodes publiques ne nécessitant aucune clé API, sûres pour les navigateurs et les clients mobiles.
- `ModerationApi` - Une suite complète d'API de modération en temps réel et rapide. Chaque méthode `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.

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