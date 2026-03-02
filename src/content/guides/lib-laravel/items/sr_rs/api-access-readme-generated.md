### Putem Facade

```php
use FastComments\Laravel\Facades\FastComments;

// Admin API (zahteva API ključ)
$comments = FastComments::admin()->getComments('tenant-id');

// Public API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// SSO (jedinstvena prijava)
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### Putem injektovanja zavisnosti

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

### Direktan pristup SDK-u

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