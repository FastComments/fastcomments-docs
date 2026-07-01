```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Οι βοηθοί SSO περιλαμβάνονται στο πακέτο (`package:fastcomments_dart/sso/...`).

Ο πελάτης εκθέτει τρεις κλάσεις API:

- `DefaultApi` - Μέθοδοι αυθεντικοποιημένα με κλειδί API για χρήση στην πλευρά του διακομιστή.
- `PublicApi` - δημόσιες μέθοδοι που δεν απαιτούν κλειδί API, ασφαλείς για περιηγητές και
  κινητές εφαρμογές.
- `ModerationApi` - μια εκτεταμένη συλλογή από ζωντανά και γρήγορα APIs διαχείρισης. Κάθε μέθοδος `ModerationApi` λαμβάνει μια παράμετρο `sso` και μπορεί να αυθεντικοποιηθεί μέσω SSO ή cookie συνεδρίας FastComments.com.

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