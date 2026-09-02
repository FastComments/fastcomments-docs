---
와일드카드를 사용하여 특정 이메일 제공자를 사용하는 사용자를 차단할 수 있습니다.

예를 들어, **@bademail.com**에서 온 모든 댓글이 스팸임을 발견하면, 차단된 사용자를 추가할 때 이메일 입력 필드에 "*@bademail.com"을 입력하여 해당 전체 이메일 제공자를 간단히 차단할 수 있습니다.

이메일에서 @ 앞에 있는 "*"를 주의하세요.

### Subdomains

도메인 차단은 해당 도메인의 모든 하위 도메인도 포함합니다. `*@bademail.com`을 차단하면 `someone@mail.bademail.com`와 `someone@eu.mail.bademail.com`도 차단되므로 각 하위 도메인마다 별도로 차단을 추가할 필요가 없습니다.

특정 하위 도메인만 차단하려면 해당 하위 도메인을 입력하십시오. 예를 들어 `*@mail.bademail.com`. 이 차단은 `someone@bademail.com`에는 영향을 주지 않습니다.

### Banning a Domain From a Comment

패턴을 직접 입력할 필요가 없습니다. Moderate Comments 페이지에서 댓글의 사용자를 차단할 때, 차단 대화 상자에 "Ban All @domain Users" 체크박스가 있어 댓글 작성자의 이메일 도메인에 대해 동일한 `*@domain` 차단을 생성합니다.

### Supported Patterns

지원되는 와일드카드 형식은 전체 이름 부분을 대체하는 단일 `*` 뒤에 `@`와 도메인이 오는 형태뿐입니다. 다른 형태는 저장하려고 할 때 거부됩니다:

- `*@*.bademail.com`은 필요하지 않습니다. `*@bademail.com`이 이미 하위 도메인을 포함하기 때문입니다.
- `name*@bademail.com` 및 `*bademail.com`은 지원되지 않습니다.

---