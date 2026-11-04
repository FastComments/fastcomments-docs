`settings.py`에서 SSO를 활성화하고 모드를 선택하세요. Secure SSO는 API 시크릿을 사용하여 HMAC-SHA256으로 서버 측에서 사용자를 서명하며, 권장됩니다.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # your API secret; signs Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Map FastComments fields to your user model. Values may be an attribute
        # name, a dotted path ("profile.avatar_url"), a callable(user), or None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, or dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, or dotted path
    },
}
```

> **SSO `id`는 신중하게 선택하세요.** FastComments의 `id`는 사용자의 댓글 기록에 대한 영구 식별자입니다. 기본 `USER_MAP`은 이를 Django 기본 키에 매핑하여 설정 없이 바로 사용할 수 있게 해주지만, 순차적인 정수 PK는 추측 가능하며 나중에 변경하기 어렵습니다(사용자의 `id`를 변경하면 기록이 새 계정으로 분리됩니다). 데모 이상의 용도로는 `id`를 미리 선택한 안정적이고 불투명한 값(UUID 또는 전용 공개 id)에 매핑하고, 개인 데이터를 절대 포함하지 마세요. 예제 앱은 이러한 이유로 사용자 이름 기반 id를 사용합니다.

SSO는 현재 사용자를 위해 `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, 그리고
`{% fastcomments_user_activity %}`에 자동으로 주입됩니다.

로그아웃된 방문자에게 표시되는 로그인/로그아웃 URL은 기본적으로 `reverse("login")` /
`reverse("logout")`이며, `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`로 재정의할 수 있습니다.

### 커스텀 매핑

`USER_MAP`보다 높은 우선순위를 가지는 두 가지 옵션이 있습니다:

- **사용자 모델의 메서드** (인터페이스의 Python적 유사체):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **전역 매퍼**, `callable(user) -> dict`에 대한 점으로 구분된 경로:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

우선순위는 `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP` 순입니다.