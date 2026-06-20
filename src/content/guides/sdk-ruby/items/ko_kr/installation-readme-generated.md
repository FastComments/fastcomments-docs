---
애플리케이션의 Gemfile에 다음 줄을 추가하세요:

```ruby
gem 'fastcomments'
```

그런 다음 실행하세요:

```bash
bundle install
```

또는 직접 설치하려면:

```bash
gem install fastcomments
```

### 라이브러리 내용

이 라이브러리에는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티가 포함되어 있습니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 공개 API와 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, 그리고 `ModerationApi`의 세 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 장치 등에서 인증 없이 직접 호출할 수 있는 API 호출을 포함합니다. `ModerationApi`는 모더레이터 대시보드를 구동하는 메서드를 포함합니다.

`ModerationApi`는 댓글 관리(목록, 카운트, 검색, 로그, 내보내기), 관리 작업(제거/복원, 플래그, 검토/스팸/승인 상태 설정, 투표, 스레드 재개/종료), 금지(댓글로부터 금지, 취소, 사전 금지 요약, 금지 상태/환경설정, 금지된 사용자 수), 및 배지 및 신뢰(배지 수여/제거, 수동 배지, 신뢰도 가져오기/설정, 사용자 내부 프로필)를 다룹니다. 각 `ModerationApi` 메서드는 `sso` 매개변수를 받아 SSO로 인증된 모더레이터를 대신하여 요청을 수행할 수 있게 합니다.
---