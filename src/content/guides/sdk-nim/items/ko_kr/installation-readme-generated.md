---
### Nimble 사용하기

```bash
nimble install fastcomments
```

### 소스에서 빌드하기

```bash
nimble build
```

### 라이브러리 내용

이 라이브러리에는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티가 포함되어 있습니다.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `api_default`, `api_public`, `api_moderation`의 세 가지 API 모듈이 있습니다. `api_default`는 API 키가 필요한 메서드를 포함하고, `api_public`은 인증 없이 브라우저/모바일 장치 등에서 직접 호출할 수 있는 API 호출을 포함합니다. `api_moderation` 모듈에는 관리자 대시보드를 위한 메서드가 포함되어 있습니다.

`api_moderation` 메서드는 댓글 및 해당 로그의 목록화, 개수 집계, 검색 및 내보내기; 댓글 제거/복원, 플래그 지정, 검토/스팸/승인 상태 설정, 투표 조정, 스레드 재개방/닫기 같은 중재 작업; 차단(댓글에서 사용자를 차단, 차단 취소, 사전 차단 요약, 차단 상태 및 환경설정, 차단된 사용자 수); 그리고 배지 및 신뢰도(배지 수여/제거, 수동 배지 목록, 사용자의 신뢰도 가져오기/설정, 사용자의 내부 프로필 가져오기)를 다룹니다. 모든 `api_moderation` 메서드는 호출이 SSO 관리자 자격으로 인증되도록 `sso` 매개변수를 허용합니다.
---