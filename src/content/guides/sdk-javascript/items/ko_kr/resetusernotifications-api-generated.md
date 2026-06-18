## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니오 |  |
| afterCreatedAt | number | 아니오 |  |
| unreadOnly | boolean | 아니오 |  |
| dmOnly | boolean | 아니오 |  |
| noDm | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## 예제

[inline-code-attrs-start title = 'resetUserNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b1f2";
const afterId: string = "notification_0001";
const afterCreatedAt: number = Date.now() - 60 * 60 * 1000; // 한 시간 전
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const sso: string = "sso_session_7f2d";
const result: ResetUserNotifications200Response = await resetUserNotifications(
  tenantId,
  afterId,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  sso
);
[inline-code-end]