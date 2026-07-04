Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC-SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # API 비밀 키; Secure SSO에 서명합니다
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # FastComments 필드를 사용자 모델에 매핑합니다. 값은 속성 이름, 점 표기 경로("profile.avatar_url"), callable(user) 또는 None이 될 수 있습니다.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, 또는 점 표기 경로
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, 또는 점 표기 경로
    },
}
```

> **SSO `id`를 신중하게 선택하십시오.** FastComments `id`는 사용자의 댓글 기록에 대한 영구적인 핸들입니다. 기본 `USER_MAP`은 편리성을 위해 이를 Django 기본키에 매핑하지만, 순차적인 정수 PK는 열거 가능하고 나중에 변경하기 어렵습니다(사용자의 `id`를 변경하면 기록이 새 계정으로 분리됩니다). 데모를 넘어서는 경우, `id`를 사전에 선택한 안정적이고 불투명한 값(예: UUID 또는 전용 공개 ID)으로 매핑하고 절대 비공개 데이터를 넣지 마세요. 예제 앱은 이 이유로 사용자 이름 기반 ID를 사용합니다.

SSO는 현재 사용자를 위해 `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, 그리고
`{% fastcomments_user_activity %}`에 자동으로 주입됩니다.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### 사용자 정의 매핑

Two higher-precedence options beat `USER_MAP`:

- **사용자 모델에 정의된 메서드** (인터페이스와 유사한 파이썬 방식):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **전역 매퍼**, `callable(user) -> dict`에 대한 점 표기 경로:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

`USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP` 순으로 우선순위가 적용됩니다.