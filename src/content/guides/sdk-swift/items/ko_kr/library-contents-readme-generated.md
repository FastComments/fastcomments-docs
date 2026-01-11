FastComments Swift SDK는 여러 모듈로 구성되어 있습니다:

- **Client Module** - FastComments REST API용 자동 생성 API 클라이언트
  - 모든 API 모델에 대한 완전한 타입 정의
  - 인증된 (`DefaultAPI`) 및 공개 (`PublicAPI`) 엔드포인트
  - 완전한 async/await 지원
  - 자세한 API 문서는 [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) 참조

- **SSO Module** - 서버 측 Single Sign-On 유틸리티
  - 사용자 인증을 위한 보안 토큰 생성
  - 단순 및 보안 SSO 모드 모두 지원
  - CryptoKit을 사용한 HMAC-SHA256 기반 토큰 서명