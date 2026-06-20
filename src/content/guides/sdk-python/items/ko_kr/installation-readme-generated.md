### PyPI

```bash
pip install fastcomments
```

### 라이브러리 구성

이 라이브러리는 두 개의 모듈로 구성되어 있습니다: 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 수작업 유틸리티를 포함한 코어 Python 라이브러리(SSO 지원 포함).

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [코어 라이브러리 문서 (SSO 예제 포함)](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi`의 세 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 메서드를 포함합니다. `ModerationApi`는 모더레이터 대시보드를 구동하며 댓글 관리용 메서드(목록, 카운트, 검색, 로그, 내보내기), 중재 작업(제거/복원, 플래그, 검토/스팸/승인 상태 설정, 투표, 쓰레드 재개/종료), 밴(댓글로부터 밴, 취소, 사전 밴 요약, 밴 상태/설정, 밴된 사용자 수), 그리고 배지 및 신뢰도(배지 수여/제거, 수동 배지, 신뢰도 가져오기/설정, 사용자 내부 프로필)를 포함합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 허용하므로 SSO로 인증된 모더레이터를 대신해 호출할 수 있습니다.