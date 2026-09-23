## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## 응답

반환: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## 예시

[inline-code-attrs-start title = 'getV1PageLikes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const urlId: string = "https://myblog.com/post/42";
  const ssoToken: string = "sso_user_987";

  const likesWithSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId, ssoToken);
  const likesWithoutSSO: GetV1PageLikes = await getV1PageLikes(tenantId, urlId);
})();
[inline-code-end]