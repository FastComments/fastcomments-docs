### Απλό SSO

Το Απλό SSO είναι απλό στη χρήση, αλλά παρέχει λιγότερη ασφάλεια σε σχέση με το Ασφαλές SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Δημιουργία δεδομένων χρήστη
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Με σύνδεση/αποσύνδεση βάσει URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ή με callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Χειρισμός σύνδεσης */ },
    function($url) { /* Χειρισμός αποσύνδεσης */ }
);

// Λάβετε το token για να το περάσετε στο FastComments
$token = $sso->prepareToSend();
```

### Ασφαλές SSO

Το Ασφαλές SSO παρέχει αυξημένη ασφάλεια με επαλήθευση HMAC:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Δημιουργία δεδομένων χρήστη
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Προσθήκη προαιρετικών δεδομένων αν χρειάζεται
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Δημιουργία του αντικειμένου SSO με το API κλειδί σας
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Λάβετε το token για να το περάσετε στο FastComments
$token = $sso->prepareToSend();
```