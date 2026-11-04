애플리케이션의 Gemfile에 다음 줄을 추가하세요:

```ruby
gem 'fastcomments'
```

그 다음 실행하세요:

```bash
bundle install
```

또는 직접 설치하려면:

```bash
gem install fastcomments
```

### 라이브러리 구성

이 라이브러리는 생성된 API 클라이언트와 API 작업을 더 쉽게 만들어주는 SSO 유틸리티를 포함합니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 공개 API vs 보안 API

API 클라이언트에는 `DefaultApi`, `PublicApi`, `ModerationApi` 세 가지 클래스가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 인증 없이 브라우저/모바일 기기 등에서 직접 호출할 수 있는 API 호출을 포함합니다. `ModerationApi`는 중재자 대시보드를 구동하는 메서드를 포함합니다.

`ModerationApi`는 광범위한 실시간 고속 중재 API 모음을 제공합니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받으며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.