### Facade Üzerinden

```php
use FastComments\Laravel\Facades\FastComments;

// Yönetici API'si (API anahtarı gerektirir)
$comments = FastComments::admin()->getComments('tenant-id');

// Herkese Açık API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// Tek Oturum Açma (SSO)
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### Bağımlılık Enjeksiyonu ile

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

### Doğrudan SDK Erişimi

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