이 SDK는 최적의 호환성을 보장하고 런타임 오류를 방지하기 위해 **이중 진입점**을 사용합니다:

- **`fastcomments-sdk/browser`** - 네이티브 `fetch`를 사용하는 브라우저 안전 버전
- **`fastcomments-sdk/server`** - SSO를 지원하는 전체 Node.js 버전
- **`fastcomments-sdk`** (기본) - 타입 전용, 어디서나 안전하게 임포트 가능