req
tenantId
urlId

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | number | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | number | 否 |  |
| skipChildren | number | 否 |  |
| limit | number | 否 |  |
| limitChildren | number | 否 |  |
| countChildren | boolean | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | boolean | 否 |  |
| countAll | boolean | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeNotificationCount | boolean | 否 |  |
| asTree | boolean | 否 |  |
| maxTreeDepth | number | 否 |  |
| useFullTranslationIds | boolean | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | Array<string> | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |

## 响应

返回：[`GetCommentsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublicResponse.ts)

## 示例

[inline-code-attrs-start title = 'getCommentsPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---