---
FastComments Rust SDK는 여러 모듈로 구성됩니다:

- **Client Module** - FastComments REST API용 API 클라이언트
  - 모든 API 모델에 대한 완전한 타입 정의
  - FastComments 모든 메서드를 포괄하는 세 개의 API 클라이언트:
    - `default_api` (**DefaultApi**) - 서버 측 사용을 위한 API 키 인증 메서드
    - `public_api` (**PublicApi**) - 브라우저 및 모바일 앱에서 호출해도 안전한 공개, API 키가 필요 없는 메서드
    - `moderation_api` (**ModerationApi**) - 실시간 및 빠른 중재 API의 광범위한 제품군. 모든 Moderation 메서드는 `sso` 매개변수를 받아 SSO 또는 FastComments.com 세션 쿠키로 인증할 수 있습니다.
  - tokio를 이용한 완전한 async/await 지원
  - 자세한 API 문서는 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)를 참조하십시오.

- **SSO Module** - 서버 측 싱글 사인온 유틸리티
  - 사용자 인증을 위한 안전한 토큰 생성
  - 단순 및 보안 SSO 모드 모두 지원
  - HMAC-SHA256 기반 토큰 서명

- **Core Types** - 공유 타입 정의 및 유틸리티
  - 댓글 모델 및 메타데이터 구조
  - 사용자 및 테넌트 구성
  - 공통 작업을 위한 도우미 함수
---