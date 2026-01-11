### SSO פשוט

SSO פשוט קל לשימוש, אך מספק אבטחה פחותה בהשוואה ל-SSO מאובטח:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Create user data
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// With URL-based login/logout
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Or with callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* טפל בהתחברות */ },
    function($url) { /* טפל בהתנתקות */ }
);

// Get the token to pass to FastComments
$token = $sso->prepareToSend();
```

### SSO מאובטח

SSO מאובטח מספק אבטחה משופרת באמצעות אימות HMAC:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Create user data
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Add optional data if needed
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Create the SSO object with your API key
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Get the token to pass to FastComments
$token = $sso->prepareToSend();
```