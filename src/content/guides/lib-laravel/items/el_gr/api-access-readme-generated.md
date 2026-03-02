### Μέσω Facade

```php
use FastComments\Laravel\Facades\FastComments;

// Admin API (απαιτεί κλειδί API)
$comments = FastComments::admin()->getComments('tenant-id');

// Public API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// SSO
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### Μέσω Dependency Injection

```php
use FastComments\Laravel\FastCommentsManager;

class CommentController extends Controller
{
    public function index(FastCommentsManager $fc)
    {
        $comments = $fc->admin()->getComments($fc->tenantId());
        // ...
    }
}
```

### Άμεση Πρόσβαση στο SDK

```php
use FastComments\Client\Api\DefaultApi;

class CommentController extends Controller
{
    public function index(DefaultApi $api)
    {
        $comments = $api->getComments('tenant-id');
        // ...
    }
}
```