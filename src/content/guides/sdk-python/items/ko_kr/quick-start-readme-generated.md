### 인증된 API 사용 (DefaultApi)

**Important:** 인증된 요청을 하기 전에 Configuration에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류와 함께 실패합니다.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API 클라이언트를 생성하고 구성합니다
config = Configuration()
config.host = "https://fastcomments.com"

# 필수: API 키 설정 (FastComments 대시보드에서 확인)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# 구성된 클라이언트로 API 인스턴스를 생성합니다
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 이제 인증된 API 호출을 할 수 있습니다
try:
    # 예시: SSO 사용자 추가
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 일반적인 오류:
    # - 401: API 키가 없거나 유효하지 않음
    # - 400: 요청 검증 실패
```

### 공개 API 사용 (PublicApi)

공개 엔드포인트는 인증이 필요하지 않습니다:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 모더레이션 대시보드 사용 (ModerationApi)

`ModerationApi`는 모더레이터 대시보드를 구동합니다. 메서드는 `sso` 토큰을 전달하여 모더레이터를 대신해 호출됩니다:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # 모더레이션 대기 중인 댓글 수를 가져옵니다
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Single Sign-On) 사용

SDK에는 보안 SSO 토큰을 생성하는 유틸리티가 포함되어 있습니다:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 사용자 데이터 생성 (id, email, username은 필수)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API 비밀키로 서명 (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# 위젯이나 API 호출에 전달할 SSO 토큰 생성
sso_token = sso.create_token()

# 프론트엔드에서 이 토큰을 사용하거나 API 호출에 전달
print(f"SSO Token: {sso_token}")
```

간단한 SSO (보안 수준 낮음, 테스트용):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 실시간 구독 (PubSub)

`pubsub` 모듈을 사용하면 WebSocket을 통해 실시간 댓글 이벤트(새 댓글, 투표, 편집, 알림 등)에 구독할 수 있으며, FastComments Java SDK의 `LiveEventSubscriber`와 유사합니다. 이를 사용하려면 `pubsub` 추가 패키지가 필요하며, 이는 생성된 API 클라이언트 위에 WebSocket 클라이언트를 추가합니다:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # 예: 이 세션에 대한 UUID
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # EU 지역을 위해 "eu" 로 설정
)

# ...나중에 더 이상 업데이트를 원하지 않을 때:
result.close()
```

구독자는 백그라운드 데몬 스레드에서 연결을 실행하고, 지터를 사용해 투명하게 재연결하며, 재연결 시 이벤트 로그 엔드포인트에서 놓친 이벤트를 가져옵니다. 선택적 `can_see_comments` 콜백(`List[str] -> Dict[str, str]`, 사용자가 **볼 수 없는** ID를 반환)을 전달하여 사용자가 볼 수 없는 댓글 이벤트를 필터링할 수 있습니다. `disable_live_commenting=True` 로 설정하면 `subscribe_to_changes` 가 `None` 을 반환하는 no‑op이 됩니다.

### 일반적인 문제

1. **401 "missing-api-key" 오류**: DefaultApi 인스턴스를 만들기 전에 `config.api_key = {"api_key": "YOUR_KEY"}` 를 설정했는지 확인하세요.
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `DefaultApi`, 클라이언트/공개 요청에는 `PublicApi`, 모더레이터 대시보드 요청에는 `ModerationApi` 를 사용하세요.
3. **Import 오류**: 올바른 모듈에서 가져오는지 확인하세요:
   - API 클라이언트: `from client import ...`
   - SSO 유틸리티: `from sso import ...`
   - 실시간 구독: `from pubsub import ...` (`pubsub` 추가 패키지가 필요)