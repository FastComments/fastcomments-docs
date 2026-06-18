## 매개변수

| 이름 | 타입 | 필수 | 설명 |
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
const tenantId: string = 'tenant_9f8b7c6a';
const userId: string = 'user_5a4b3c2d';
const urlId: string = 'post_84f2a1b9';
const fromCommentId: string = 'cmt_0a1b2c3d';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 0;

const notifications: GetNotifications200Response = await getNotifications(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type,
  skip
);
[inline-code-end]

---