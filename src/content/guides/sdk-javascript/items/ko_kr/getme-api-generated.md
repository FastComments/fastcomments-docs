사용 중인 자격 증명을 식별합니다: 해당 자격 증명이 속한 테넌트와 OAuth 토큰인 경우 이를 승인한 사용자를 식별합니다.  
통합에서는 이를 사용하여 연결을 테스트하고 라벨을 지정합니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 응답

반환: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## 예시

[inline-code-attrs-start title = 'getMe 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]

---