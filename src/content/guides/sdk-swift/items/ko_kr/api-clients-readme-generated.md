The FastComments SDK는 세 개의 API 클라이언트를 제공합니다:

### PublicAPI - 클라이언트 안전 메서드

`PublicAPI`는 클라이언트 측 코드(iOS/macOS 앱)에서 호출해도 안전한 메서드를 포함합니다. 이러한 메서드:
- API 키가 필요하지 않습니다
- 인증을 위해 SSO 토큰을 사용할 수 있습니다
- 사용자/디바이스당 속도 제한이 적용됩니다
- 최종 사용자용 애플리케이션에 적합합니다

**Example use case**: iOS 앱에서 댓글을 가져오고 생성하기

### DefaultAPI - 서버 측 메서드

`DefaultAPI`는 인증된 메서드를 포함하며 API 키가 필요합니다. 이러한 메서드:
- FastComments API 키가 필요합니다
- 서버 측 코드에서만 호출되어야 합니다
- FastComments 데이터에 대한 전체 접근 권한을 제공합니다
- 테넌트당 속도 제한이 적용됩니다

**Example use case**: 관리 작업, 대량 데이터 내보내기, 사용자 관리

### ModerationAPI - 중재자 대시보드 메서드

`ModerationAPI`는 실시간 및 빠른 중재 API의 광범위한 스위트를 제공합니다. 모든 `ModerationAPI` 메서드는 `sso` 매개변수를 받아들이며 SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.

**Example use case**: 커뮤니티 중재자를 위한 중재 경험 구축

**IMPORTANT**: 절대 클라이언트 측 코드에 API 키를 노출하지 마세요. API 키는 서버 측에서만 사용해야 합니다.