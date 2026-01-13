로컬 개발의 경우 [ngrok](https://ngrok.com/)와 같은 도구를 사용하세요.

시스템 보안을 단순화하기 위해 로컬 개발은 다른 환경을 설정하고 보안을 유지하는 것과 동일한 절차를 따릅니다. 

### Step 1: Add "localhost" to domains in your account.

"localhost"를 [여기에 도메인으로 추가하세요](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

도메인에 대한 웹훅 구성을 추가할 예정이므로 API 키가 필요합니다. [여기에서 할 수 있습니다.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

운영 도메인에 대해 API Secret이 정의되어 있는지 확인하세요. 다른 모든 도메인의 이벤트는 와일드카드(테스트) 시크릿을 사용합니다.

### Step 3: Add Your Webhook

ngrok 또는 유사한 도구를 실행하는 동안 "localhost"의 값을 [여기](https://fastcomments.com/auth/my-account/manage-data/webhooks)에서 설정하세요.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Step 4: Add A Comment

이제 댓글을 추가, 편집 또는 삭제할 수 있으며 테스트용 API 키를 사용하여 이벤트가 로컬 개발 머신으로 전달되는 것을 볼 수 있어야 합니다. 최대 30초의 지연
이벤트가 머신에 도달하는 데 있을 수 있습니다.