로컬 개발 시에는 [ngrok](https://ngrok.com/)과 같은 도구를 사용하세요.

시스템 보안을 간편하게 유지하기 위해, 로컬 개발은 다른 환경을 설정하고 보호하는 것과 동일한 절차를 따릅니다.

### 1단계: 계정의 도메인에 "localhost" 추가하기

"localhost"를 [여기서 도메인으로 추가](https://fastcomments.com/auth/my-account/configure-domains)하세요.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='계정 설정의 도메인 추가 양식으로, 도메인 이름 필드에 localhost가 입력되어 있음'; title='localhost 추가'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### 2단계: API 키 선택하기

도메인에 대한 웹훅 설정을 추가할 예정이므로 API 키가 필요합니다. [여기서 생성할 수 있습니다.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='새 API 시크릿 양식으로, 연결된 도메인이 localhost로 설정되고 키 이름이 Testing으로 지정되어 있음'; title='Testing API 키 추가'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

"Associate with domain"에서 "localhost" 도메인을 선택하세요.

**참고: 또는 모든 테스트 활동과 스테이징 환경에 하나의 API 시크릿을 사용할 수 있습니다. "All Domains"에 API 시크릿을 추가하고 "test"와 같은 이름을 지정하기만 하면 됩니다.**

프로덕션 도메인에 대해 API 시크릿이 정의되어 있는지 확인하세요. 다른 모든 도메인의 이벤트는 와일드카드(테스트) 시크릿을 사용합니다.

### 3단계: 웹훅 추가하기

ngrok 또는 유사한 도구가 실행 중인 상태에서 [여기서](https://fastcomments.com/auth/my-account/manage-data/webhooks) "localhost"의 값을 설정하세요.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='웹훅 관리 화면으로, localhost 도메인이 선택되고 댓글 생성 엔드포인트에 ngrok URL이 입력되어 있음'; title='테스트 웹훅 추가'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

`Send Test Payload`를 클릭하면 API 키 검증이 올바르게 이루어지는지 확인하기 위해 두 개의 테스트 이벤트를 전송합니다.

검증이 완료되면 `Save`를 누르세요.

### 4단계: 댓글 추가하기

이제 댓글을 추가, 수정 또는 삭제할 수 있으며, 테스트 API 키를 사용하여 이벤트와 함께 로컬 개발 머신으로 호출되는 것을 확인할 수 있습니다. 이벤트가 머신에 도달하는 데 최대 30초의 지연이 있을 수 있습니다.