```dart
import 'package:fastcomments_dart/sso/fastcomments_sso.dart';
import 'package:fastcomments_dart/sso/secure_sso_user_data.dart';

final sso = FastCommentsSSO.secure(
  apiKey: 'YOUR_API_SECRET',
  user: SecureSSOUserData(
    id: 'user-123',
    email: 'jane@example.com',
    username: 'jane',
    avatar: 'https://example.com/jane.png',
  ),
);
final token = sso.prepareToSend();
```