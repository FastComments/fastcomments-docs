주어진 댓글들을 위젯 로케일로 번역합니다. 일치하는 위젯 커스터마이징 규칙의 Comment Translation이 Automatic 또는 Manual로 설정된 경우에만 사용할 수 있습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| commentTranslationsRequest | CommentTranslationsRequest | 예 |  |
| sso | string | 아니요 |  |
| locale | string | 아니요 |  |

## 응답

반환값: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'translateComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'blog-post-42';
const sso: string = 'JWT_TOKEN_HERE';

const commentTranslationsRequest: CommentTranslationsRequest = {
  commentIds: ['c_98765', 'c_54321'],
  targetLocale: 'es'
};

const locale: string = 'es-ES';

const response: CommentTranslationsResponse = await translateComments(
  tenantId,
  urlId,
  commentTranslationsRequest,
  sso,
  locale
);

console.log(`${response.translations.length}개의 댓글이 번역되었습니다`);
[inline-code-end]