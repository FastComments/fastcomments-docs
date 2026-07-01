req  
tenantId  
urlId  

## פרמטרים  

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | number | לא |  |
| skipChildren | number | לא |  |
| limit | number | לא |  |
| limitChildren | number | לא |  |
| countChildren | boolean | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | boolean | לא |  |
| countAll | boolean | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| asTree | boolean | לא |  |
| maxTreeDepth | number | לא |  |
| useFullTranslationIds | boolean | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | Array<string> | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה  

מחזיר: [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## דוגמה  

[inline-code-attrs-start title = 'getCommentsPublic דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
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