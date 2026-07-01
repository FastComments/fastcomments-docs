```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Οι βοηθητικές λειτουργίες SSO περιλαμβάνονται στο πακέτο (`package:fastcomments_dart/sso/...`).

Ο πελάτης εκθέτει τρεις κλάσεις API:

- `DefaultApi` - Μέθοδοι αυθεντικοποιημένοι με API key για χρήση στο διακομιστή.
- `PublicApi` - Δημόσιες μέθοδοι που δεν απαιτούν API key, ασφαλείς για προγράμματα περιήγησης και κινητές εφαρμογές.
- `ModerationApi` - Ένα εκτενές σύνολο ζωντανών και γρήγορων API διαχείρισης. Κάθε μέθοδος του `ModerationApi` λαμβάνει μια παράμετρο `sso` και μπορεί να αυθεντικοποιηθεί μέσω SSO ή μέσω cookie συνεδρίας του FastComments.com.

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