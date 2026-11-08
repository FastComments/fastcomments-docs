### सरल SSO

सरल SSO का उपयोग करना आसान है, लेकिन यह Secure SSO की तुलना में कम सुरक्षा प्रदान करता है:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// यूज़र डेटा बनाएँ
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// URL-आधारित लॉगिन/लॉगआउट के साथ
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// या कॉलबैक के साथ
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* लॉगिन हैंडल करें */ },
    function($url) { /* लॉगआउट हैंडल करें */ }
);

// FastComments को भेजने के लिए टोकन प्राप्त करें
$token = $sso->prepareToSend();
```

### सुरक्षित SSO

सुरक्षित SSO HMAC सत्यापन के साथ बेहतर सुरक्षा प्रदान करता है:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// यूज़र डेटा बनाएँ
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// आवश्यकता हो तो वैकल्पिक डेटा जोड़ें
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// अपनी API key के साथ SSO ऑब्जेक्ट बनाएँ
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// FastComments को भेजने के लिए टोकन प्राप्त करें
$token = $sso->prepareToSend();
```