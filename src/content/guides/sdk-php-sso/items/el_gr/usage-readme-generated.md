### Απλό SSO

Το Απλό SSO είναι απλό στη χρήση, αλλά παρέχει λιγότερη ασφάλεια από το Ασφαλές SSO:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Δημιουργία δεδομένων χρήστη
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// Με σύνδεση/αποσύνδεση με βάση URL
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Ή με callbacks
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Διαχείριση εισόδου */ },
    function($url) { /* Διαχείριση αποσύνδεσης */ }
);

// Λήψη του token για να περαστεί στο FastComments
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

// Προσθήκη προαιρετικών δεδομένων εφόσον χρειάζεται
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// Δημιουργία του αντικειμένου SSO με το API key σας
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// Λήψη του token για να περαστεί στο FastComments
$token = $sso->prepareToSend();
```