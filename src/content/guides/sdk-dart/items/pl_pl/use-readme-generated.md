```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Pomocnicze funkcje SSO są zawarte w pakiecie (`package:fastcomments_dart/sso/...`).

Klient udostępnia trzy klasy API:

- `DefaultApi` - Metody uwierzytelniane kluczem API do użytku po stronie serwera.
- `PublicApi` - publiczne metody, które nie wymagają klucza API, bezpieczne dla przeglądarki i klientów mobilnych.
- `ModerationApi` - rozbudowany zestaw API do szybkiej i bieżącej moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com.

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