---
[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

댓글 위젯이 예를 들어 테넌트 ID가 "demo"인 상태로 사용될 수 있다는 것을 알 수 있습니다:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

이것은 댓글 위젯을 시험해 보거나 가지고 놀아보기 위한 목적일 뿐입니다. 실제 운영 환경에서는 다음과 같이 자신의 테넌트 ID를 전달해야 합니다:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

귀하의 테넌트 ID는 계정의 댓글 위젯 <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">코드 스니펫</a>에 이미 적용되어 있습니다.

또한 테넌트 ID를 확인하고 API 키를 관리하려면 [API 자격 증명 페이지](https://fastcomments.com/auth/my-account/api-secret)를 확인하세요.

앞으로는 FastComments에 로그인한 상태라면(https://fastcomments.com에 로그인한 경우) 코드 예제들이 실제 테넌트 ID를 사용합니다.

---