## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| tag | string | Evet |  |
| updateHashTagBody | UpdateHashTagBody | Hayır |  |

## Yanıt

Döndürür: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Örnek

[inline-code-attrs-start title = 'patchHashTag Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Opsiyonel gövde olmadan çağır
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Güncelleme için gövde hazırla
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]