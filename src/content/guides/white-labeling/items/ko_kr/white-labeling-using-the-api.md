---
API를 사용하여 화이트 라벨 테넌트를 생성하려면 다음을 수행해야 합니다:

1. 테넌트를 생성하려면 [Tenants API](/guide-api.html#tenant-structure)를 호출합니다.
2. 테넌트용 하나 이상의 패키지를 생성하려면 [TenantPackages API](/guide-api.html#tenant-package-structure)를 호출합니다.
3. 테넌트에서 활성화된 패키지를 정의하려면 [Tenants API](/guide-api.html#tenant-patch)를 호출합니다.
4. 테넌트에 관리자 사용자를 추가하려면 [TenantUsers API](/guide-api.html#tenant-user-structure)를 호출합니다.
5. 테넌트에 모더레이터를 추가하고 초대하려면 [Moderators API](/guide-api.html#moderator-structure)를 호출합니다.
6. 선택적으로, [Setup SSO](/guide-customizations-and-configuration.html#sso).

이 모든 작업은 체험 기간 내에 수행할 수 있습니다.
---