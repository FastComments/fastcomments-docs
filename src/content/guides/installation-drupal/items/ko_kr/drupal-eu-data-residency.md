만약 FastComments 계정이 EU에 호스팅되어 있다면, `Administration > Configuration > Content > FastComments`에서 두 가지 설정을 업데이트하세요:

- **CDN URL** - `https://cdn-eu.fastcomments.com`
- **Site URL** - `https://eu.fastcomments.com`

또한 기본 US 대시보드 대신 EU 대시보드인 [eu.fastcomments.com/auth/my-account/api](https://eu.fastcomments.com/auth/my-account/api)에서 Tenant ID와 API Secret을 가져와야 합니다. 모듈의 다른 모든 부분은 동일하게 작동합니다.