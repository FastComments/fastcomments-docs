The FastComments Rust SDK는 여러 모듈로 구성됩니다:

- **Client Module** - FastComments REST API를 위한 API 클라이언트
  - 모든 API 모델에 대한 완전한 타입 정의
  - 모든 FastComments 메서드를 포함하는 세 가지 API 클라이언트:
    - `default_api` (**DefaultApi**) - 서버 측 사용을 위한 API 키 인증 메서드
    - `public_api` (**PublicApi**) - 브라우저와 모바일 앱에서 안전하게 호출할 수 있는 공개(API 키 불필요) 메서드
    - `moderation_api` (**ModerationApi**) - 모더레이터 대시보드를 지원하는 메서드로, 댓글 모더레이션(목록, 수, 검색, 로그, 내보내기), 모더레이션 작업(제거/복원, 신고, 검토/스팸/승인 상태 설정, 투표, 스레드 재개/종료), 차단(댓글 기반 차단, 취소, 사전 차단 요약, 차단 상태/환경설정, 차단된 사용자 수), 및 배지와 신뢰(배지 수여/제거, 수동 배지, 신뢰도 가져오기/설정, 사용자 내부 프로필)을 포함합니다. 모든 Moderation 메서드는 `sso` 매개변수를 허용하므로 SSO 인증된 모더레이터를 대신하여 호출할 수 있습니다.
  - tokio를 사용한 완전한 async/await 지원
  - 자세한 API 문서는 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) 참조

- **SSO Module** - 서버 측 싱글 사인온 유틸리티
  - 사용자 인증을 위한 안전한 토큰 생성
  - 단순 및 보안 SSO 모드 모두 지원
  - HMAC-SHA256 기반 토큰 서명

- **Core Types** - 공유 타입 정의 및 유틸리티
  - 댓글 모델 및 메타데이터 구조
  - 사용자 및 테넌트 구성
  - 공통 작업을 위한 헬퍼 함수