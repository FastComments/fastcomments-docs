The FastComments SDK는 두 가지 유형의 API 엔드포인트를 제공합니다:

### PublicAPI - 클라이언트용 안전한 엔드포인트

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- API 키가 필요하지 않습니다
- 인증을 위해 SSO 토큰을 사용할 수 있습니다
- 사용자/기기별로 속도 제한이 적용됩니다
- 최종 사용자 대상 애플리케이션에 적합합니다

**예시 사용 사례**: iOS 앱에서 댓글을 가져오고 생성하기

### DefaultAPI - 서버 사이드 엔드포인트

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- FastComments API 키가 필요합니다
- 반드시 서버 사이드 코드에서만 호출해야 합니다
- FastComments 데이터에 대한 전체 접근 권한을 제공합니다
- 테넌트별로 속도 제한이 적용됩니다

**예시 사용 사례**: 관리 작업, 대량 데이터 내보내기, 모더레이션 도구

**중요**: 클라이언트 사이드 코드에 API 키를 절대 노출하지 마십시오. API 키는 서버 사이드에서만 사용해야 합니다.