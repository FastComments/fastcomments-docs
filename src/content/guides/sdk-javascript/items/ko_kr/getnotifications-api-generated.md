## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| urlId | string | 아니오 |  |
| fromCommentId | string | 아니오 |  |
| viewed | boolean | 아니오 |  |
| type | string | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## 예제

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b1a2c';
  const userId: string = 'user_42';
  const urlId: string = 'https://news.example.com/articles/2026/01/11/comment-thread';
  const fromCommentId: string = 'cmt_9a7b';
  const viewed: boolean = false;
  const type: string = 'mention';
  const skip: number = 0;
  const response: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
  console.log(response);
})();
[inline-code-end]

---