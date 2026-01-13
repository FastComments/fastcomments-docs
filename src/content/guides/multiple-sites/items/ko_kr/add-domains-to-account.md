FastComments는 요청이 귀하의 사이트에서 오는 것인지 확인하기 위해 귀하의 계정으로의 요청을 인증합니다. 이것이 우리가 FastComments를 설치하려는 사이트를 알아야 하는 이유입니다.

FastComments는 도메인과 서브도메인 방식의 인증을 모두 지원합니다.

예를 들어 `https://example.com` 사이트를 살펴보겠습니다. 이 경우, "`example.com`"이 도메인입니다. `example.com`은 `example.com`과 `www.example.com`을 모두 지원합니다. 여기서 "www"를 "서브도메인"이라고 부르겠습니다.

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

블로깅 플랫폼을 사용하고 있고 서브도메인이 제공되었다면, 예를 들어 `cats.blogger.com`과 같이 **서브도메인을 포함한 전체 도메인**을 계정에 추가해야 합니다.

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

체험 기간 동안에는 요청이 해당 도메인에서 올 때 **도메인이 자동으로 귀하의 계정에 추가됩니다**. 그러나 이 기간이 지나면 보안을 위해 명시적으로 추가해야 합니다. 이 자동 동작이 발생하면 이메일을 받게 됩니다.

로컬 개발을 위해 `localhost`를 추가할 필요는 없습니다 - 기본적으로 허용됩니다.

#### API를 통해

도메인은 [DomainConfigs API를 통해](/guide-api.html#domain-config-structure) 추가 및 구성할 수도 있습니다.