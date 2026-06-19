req
tenantId
urlId

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| page | number | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | number | No |  |
| skipChildren | number | No |  |
| limit | number | No |  |
| limitChildren | number | No |  |
| countChildren | boolean | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | boolean | No |  |
| countAll | boolean | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | boolean | No |  |
| includeNotificationCount | boolean | No |  |
| asTree | boolean | No |  |
| maxTreeDepth | number | No |  |
| useFullTranslationIds | boolean | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | Array<string> | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Respuesta

Devuelve: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const response: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
    'news-tenant-42',
    'article-2026-06-19-abc123',
    1,
    undefined,
    'sso_eyJhbGciOiJIUzI1Ni',
    0,
    0,
    25,
    5,
    true,
    undefined,
    true,
    false,
    true,
    'en-US',
    'reactions,moderation',
    false,
    true,
    true,
    3,
    false,
    undefined,
    'climate change',
    ['environment', 'policy'],
    'user-789',
    undefined,
    undefined,
    undefined
  );
  console.log(response);
})();
[inline-code-end]

---