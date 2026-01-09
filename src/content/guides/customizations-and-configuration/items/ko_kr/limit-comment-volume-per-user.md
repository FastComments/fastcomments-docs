---
기본적으로 각 사용자는 동일한 1분 동안 최대 `5 comments`를 제출할 수 있습니다.

이것은 user id, anon user id, and ip address (hashed)로 추적됩니다.

이는 코드 없이 위젯 맞춤 설정 페이지에서 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

참고로 comment creation API를 사용 중인 경우, 요청에 사용자의 원래 `ip` 주소를 당사 백엔드로 전달하는 것이 좋습니다. 이렇게 하면 속도 제한이 적용
사용자별로 적용되고 귀하의 계정에 전역적으로 적용되지 않습니다.

---