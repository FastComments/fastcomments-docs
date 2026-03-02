### 퍼사드(Facade)를 통한 사용

```php
use FastComments\Laravel\Facades\FastComments;

// 관리자 API (API 키 필요)
$comments = FastComments::admin()->getComments('tenant-id');

// 공개 API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// SSO
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### 의존성 주입(Dependency Injection)을 통한 사용

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

### SDK에 직접 접근

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