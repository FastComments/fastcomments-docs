req  
tenantId  
urlId  

## Parameters  

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | number | Nee |  |
| skipChildren | number | Nee |  |
| limit | number | Nee |  |
| limitChildren | number | Nee |  |
| countChildren | boolean | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | boolean | Nee |  |
| countAll | boolean | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeNotificationCount | boolean | Nee |  |
| asTree | boolean | Nee |  |
| maxTreeDepth | number | Nee |  |
| useFullTranslationIds | boolean | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | Array<string> | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Respons  

Retourneert: [`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## Voorbeeld  

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
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