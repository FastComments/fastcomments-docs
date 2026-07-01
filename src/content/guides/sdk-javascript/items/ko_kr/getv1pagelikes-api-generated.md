---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 응답

반환: [`GetV1PageLikesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikesResponse.ts)

## 예시

[inline-code-attrs-start title = 'getV1PageLikes 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPageLikes() {
  const tenantId: string = "acme-corp-tenant-42";
  const urlId: string = "blog-post-2024-06-typescript-best-practices";

  const likes: GetV1PageLikesResponse = await getV1PageLikes(tenantId, urlId);
  console.log(likes);
}
[inline-code-end]

---