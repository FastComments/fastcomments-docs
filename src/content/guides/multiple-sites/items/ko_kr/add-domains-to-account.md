FastComments는 요청이 귀하의 사이트에서 온 것인지 확인하기 위해 계정에 대한 요청을 인증합니다. 따라서 FastComments를 설치하려는 사이트(들)를 알아야 합니다.

FastComments는 도메인 및 하위 도메인을 통한 인증을 지원합니다.

`https://example.com` 사이트를 예로 들어 보겠습니다. 이 경우, "`example.com`"이 도메인입니다. `example.com`은 `example.com`과 `www.example.com` 모두를 지원합니다. 여기서 "www"를 "하위 도메인"이라고 부릅니다.

예시:

- 오직 `blog.example.com`만 허용하려면:
  - `blog.example.com`을 도메인에 추가합니다.
- `www.example.com`, `somesite.example.com`, 그리고 `example.com`을 허용하려면:
  - `example.com`을 도메인에 추가합니다.
  - 이는 계정에 **하나의 도메인**이 연결된 것으로 청구됩니다.
- 이제 와일드카드 하위 도메인을 추가할 수 있습니다. 예: *myname.vercel.app.
  - 이는 계정에 **하나의 도메인**이 연결된 것으로 청구됩니다.

블로그 플랫폼을 사용하고 하위 도메인이 제공된 경우, **하위 도메인을 포함한 전체 도메인**을 계정에 추가해야 합니다. 예: `cats.blogger.com`.

`My Domains` 페이지를 방문하고 하단의 `Add a Domain`을 클릭하여 계정에 도메인을 추가할 수 있습니다:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='계정에 있는 도메인들을 나열하고 하단에 Add a Domain 버튼이 있는 My Domains 페이지'; title='My Domains 페이지' app-screenshot-end]

체험 기간 동안, **도메인이 해당 도메인에서 요청이 올 경우 자동으로 계정에 추가됩니다**. 그러나 이후에는 보안을 위해 명시적으로 추가해야 합니다. 이 자동 동작이 발생하면 이메일을 받게 됩니다.

로컬 개발을 위해 `localhost`를 추가할 **필요가 없습니다** - 기본적으로 허용됩니다.

#### API를 통해

도메인은 [DomainConfigs API](/guide-api.html#domain-config-structure)를 통해서도 추가 및 구성할 수 있습니다.