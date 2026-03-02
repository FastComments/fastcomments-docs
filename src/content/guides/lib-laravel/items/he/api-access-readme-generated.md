### באמצעות ה-Facade

```php
use FastComments\Laravel\Facades\FastComments;

// ממשק Admin (דורש מפתח API)
$comments = FastComments::admin()->getComments('tenant-id');

// ממשק ציבורי
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// SSO
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### באמצעות הזרקת תלויות

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

### גישה ישירה ל-SDK

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