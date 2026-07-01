req
tenantId
urlId

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | number | Non |  |
| skipChildren | number | Non |  |
| limit | number | Non |  |
| limitChildren | number | Non |  |
| countChildren | boolean | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | boolean | Non |  |
| countAll | boolean | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| asTree | boolean | Non |  |
| maxTreeDepth | number | Non |  |
| useFullTranslationIds | boolean | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | Array<string> | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Renvoie : [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = 'acme-corp';
  const urlId: string = 'blog/post-789';
  const page: number = 1;
  const direction: SortDirections = SortDirections.Desc;
  const limit: number = 25;
  const includeConfig: boolean = true;
  const locale: string = 'en-GB';
  const modules: string = 'reactions,attachments';
  const isCrawler: boolean = false;
  const includeNotificationCount: boolean = true;
  const asTree: boolean = true;
  const maxTreeDepth: number = 4;
  const searchText: string = 'TypeScript';
  const hashTags: string[] = ['typescript', 'api'];
  const response: GetCommentsPublicResponse = await getCommentsPublic(
    tenantId,
    urlId,
    page,
    direction,
    undefined,
    undefined,
    undefined,
    limit,
    undefined,
    undefined,
    undefined,
    includeConfig,
    undefined,
    undefined,
    locale,
    modules,
    isCrawler,
    includeNotificationCount,
    asTree,
    maxTreeDepth,
    undefined,
    undefined,
    searchText,
    hashTags,
    undefined,
    undefined,
    undefined,
    undefined
  );
}
[inline-code-end]