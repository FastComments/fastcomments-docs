---
`demo` 테넌트를 끄면 위젯이 인증 오류로 로드되지 않을 수 있습니다. 이는 FastComments가 해당 도메인에서 귀하의 계정을 사용할 수 있도록 허용해야 함을 인식하지 못하기 때문입니다.

[여기에서 사이트를 계정에 추가하세요.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town은 여기서 다시 살펴볼 가치가 있습니다. 왜냐하면 val은 하나 이상의 호스트명으로 접근할 수 있기 때문입니다:

- 모든 HTTP val은 긴 기본 엔드포인트 `<org>--<id>.web.val.run` 를 가집니다.
- 맞춤 서브도메인을 클레임하면 `<name>.val.run` 이 추가됩니다.
- 맞춤 도메인([custom domain](https://docs.val.town/vals/http/custom-domains/))을 사용하면 세 번째가 추가됩니다.
- 브랜치는 자체 URL을 갖게 됩니다.

위젯을 제공하는 실제 호스트명을 모두 추가하세요. 설정 후에 서브도메인을 클레임하면 해당 서브도메인도 추가해야 합니다. 그렇지 않으면 위젯이 이전 URL에서는 작동하지만 새로운 URL에서는 실패합니다.
---