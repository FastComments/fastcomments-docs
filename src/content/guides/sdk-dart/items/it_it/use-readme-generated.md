```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Gli helper SSO sono inclusi nel pacchetto (`package:fastcomments_dart/sso/...`).

Il client espone tre classi API:

- `DefaultApi` - Metodi autenticati con chiave API per uso lato server.
- `PublicApi` - Metodi pubblici che non richiedono chiave API, sicuri per browser e client mobili.
- `ModerationApi` - una suite completa di API di moderazione in tempo reale e veloci. Ogni metodo `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.

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