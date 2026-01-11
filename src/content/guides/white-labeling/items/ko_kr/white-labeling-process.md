---
화이트 레이블링을 설정하는 절차는 다음과 같습니다:

1. 청구 처리를 어떻게 할지 선택합니다.
   1. FastComments Pro의 경우, 일정 수까지의 화이트 레이블 테넌트에 대해 고정 금액을 지불합니다.
   2. FastComments Flex의 경우, 각 테넌트와 해당 테넌트의 사용량에 대해 요금을 지불합니다.
   3. 두 경우 모두 각 테넌트의 한도를 설정합니다.
      1. 한도는 테넌트별로 맞춤 설정할 수 있습니다. 또한 판매하는 패키지를 업데이트하더라도 기존 고객에게 이미 제공한 가격을 변경하지 않고 업데이트할 수 있습니다.
2. FastComments.com 용어에 익숙해지십시오:
   1. `Tenant`은 "고객"입니다.
   2. `TenantUser`는 `Tenant` 내에서 특정 권한을 가진 관리자입니다.
   3. `TenantPackage`는 `Tenant`에 제공되는 한도와 가격이 설정된 패키지입니다.
3. 당사의 [API](/guide-api.html)와 통합하거나 테넌트를 온보딩하기 위해 [스크립트](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling)를 사용하세요.

---