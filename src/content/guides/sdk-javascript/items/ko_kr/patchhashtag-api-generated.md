## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| tag | string | 예 |  |
| updateHashTagBody | UpdateHashTagBody | 아니오 |  |

## 응답

반환: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## 예시

[inline-code-attrs-start title = 'patchHashTag 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // 선택적 본문 없이 호출
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // 업데이트를 위한 본문 준비
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]