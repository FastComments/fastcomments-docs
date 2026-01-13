---
애플리케이션의 Gemfile에 다음 줄을 추가하세요:

```ruby
gem 'fastcomments'
```

그리고 다음을 실행하세요:

```bash
bundle install
```

또는 직접 설치하려면 다음을 사용하세요:

```bash
gem install fastcomments
```

### 라이브러리 내용

이 라이브러리에는 생성된 API 클라이언트와 API 작업을 더 쉽게 해주는 SSO 유틸리티가 포함되어 있습니다.

- [API 클라이언트 라이브러리 문서](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 공개 API와 보안 API

API 클라이언트에는 두 개의 클래스, `DefaultApi`와 `PublicApi`가 있습니다. `DefaultApi`는 API 키가 필요한 메서드를 포함하고, `PublicApi`는 브라우저/모바일 기기 등에서 인증 없이 직접 호출할 수 있는 API 호출을 포함합니다.
---