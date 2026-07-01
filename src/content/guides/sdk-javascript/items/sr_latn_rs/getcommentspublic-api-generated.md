zahtev
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | number | Ne |  |
| skipChildren | number | Ne |  |
| limit | number | Ne |  |
| limitChildren | number | Ne |  |
| countChildren | boolean | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | boolean | Ne |  |
| countAll | boolean | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| asTree | boolean | Ne |  |
| maxTreeDepth | number | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | Array<string> | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vraća: [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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