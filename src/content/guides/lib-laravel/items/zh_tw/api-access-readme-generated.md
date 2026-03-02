### 透過 Facade

```php
use FastComments\Laravel\Facades\FastComments;

// 管理 API（需要 API 金鑰）
$comments = FastComments::admin()->getComments('tenant-id');

// 公開 API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// 單一登入 (SSO)
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### 透過依賴注入

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

### 直接存取 SDK

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