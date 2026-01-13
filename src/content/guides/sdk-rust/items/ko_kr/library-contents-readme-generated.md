The FastComments Rust SDK는 여러 모듈로 구성되어 있습니다:

- **Client Module** - FastComments REST API용 자동 생성 API 클라이언트
  - 모든 API 모델에 대한 완전한 타입 정의
  - 인증된 (`DefaultApi`) 및 공개 (`PublicApi`) 엔드포인트 모두
  - `tokio`를 이용한 완전한 async/await 지원
  - 자세한 API 문서는 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)를 참조하세요

- **SSO Module** - 서버 측 싱글 사인온(SSO) 유틸리티
  - 사용자 인증을 위한 안전한 토큰 생성
  - 단순 및 보안 SSO 모드 모두 지원
  - HMAC-SHA256 기반 토큰 서명

- **Core Types** - 공유 타입 정의 및 유틸리티
  - 댓글 모델 및 메타데이터 구조
  - 사용자 및 테넌트 구성
  - 일반 작업을 위한 헬퍼 함수
---