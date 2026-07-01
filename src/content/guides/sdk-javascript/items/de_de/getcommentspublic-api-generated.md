req  
tenantId  
urlId  

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nein |  |
| direction | SortDirections | Nein |  |
| sso | string | Nein |  |
| skip | number | Nein |  |
| skipChildren | number | Nein |  |
| limit | number | Nein |  |
| limitChildren | number | Nein |  |
| countChildren | boolean | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includeConfig | boolean | Nein |  |
| countAll | boolean | Nein |  |
| includei10n | boolean | Nein |  |
| locale | string | Nein |  |
| modules | string | Nein |  |
| isCrawler | boolean | Nein |  |
| includeNotificationCount | boolean | Nein |  |
| asTree | boolean | Nein |  |
| maxTreeDepth | number | Nein |  |
| useFullTranslationIds | boolean | Nein |  |
| parentId | string | Nein |  |
| searchText | string | Nein |  |
| hashTags | Array<string> | Nein |  |
| userId | string | Nein |  |
| customConfigStr | string | Nein |  |
| afterCommentId | string | Nein |  |
| beforeCommentId | string | Nein |  |

## Antwort

Rückgabe: [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
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