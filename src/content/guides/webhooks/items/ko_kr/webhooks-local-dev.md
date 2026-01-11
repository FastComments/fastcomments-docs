로컬 개발에는 [ngrok](https://ngrok.com/) 같은 도구를 사용하세요.

시스템 보안을 간소화하기 위해, 로컬 개발은 다른 환경을 설정하고 보호하는 것과 동일한 프로세스를 따릅니다. 

### Step 1: 계정의 도메인에 "localhost" 추가

계정 도메인에 "localhost"를 [여기](https://fastcomments.com/auth/my-account/configure-domains)에서 추가하세요.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: API 키 선택

도메인에 대한 웹후크 구성을 추가할 예정이므로 API 키가 필요합니다. [여기에서 할 수 있습니다.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

"Associate with domain"에서 "localhost" 도메인을 선택하세요.

**참고: 대안으로, 모든 테스트 활동 및 스테이징 환경에 하나의 API Secret을 사용할 수 있습니다. 단순히 "All Domains"에 대한 API Secret을 추가하고 이름을 "test"로 지정하세요.**

프로덕션 도메인에 대한 API Secret이 정의되어 있는지 확인하세요. 다른 모든 도메인의 이벤트는 와일드카드(테스트) 시크릿을 사용합니다.

### Step 3: 웹후크 추가

ngrok 또는 유사한 도구를 실행하는 동안 "localhost"의 값을 [여기](https://fastcomments.com/auth/my-account/manage-data/webhooks)에서 설정하세요.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

`Send Test Payload`를 클릭하면 API 키를 검증하는지 확인하기 위해 두 개의 테스트 이벤트를 전송합니다.

검증되면 `Save`를 누르세요.

### Step 4: 댓글 추가

이제 댓글을 추가, 편집 또는 삭제할 수 있으며, 테스트 API 키를 사용하여 이벤트로 로컬 개발 머신이 호출되는 것을 확인할 수 있습니다.  
이벤트가 머신에 도달하는 데 최대 30초의 지연이 있을 수 있습니다.