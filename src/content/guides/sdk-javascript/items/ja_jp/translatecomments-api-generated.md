指定されたコメントをウィジェットのロケールに翻訳します。対応するウィジェットカスタマイズルールの Comment Translation が Automatic または Manual に設定されている場合にのみ利用可能です。

## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| commentTranslationsRequest | CommentTranslationsRequest | はい |  |
| sso | string | いいえ |  |
| locale | string | いいえ |  |

## レスポンス

戻り値: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CommentTranslationsResponse.ts)

## 例

[inline-code-attrs-start title = 'translateComments の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

console.log(`${response.translations.length} 件のコメントを翻訳しました`);
[inline-code-end]