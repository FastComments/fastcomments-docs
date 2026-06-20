---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| textSearch | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## 例

[inline-code-attrs-start title = 'getSearchSuggest の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const textSearch: string = 'How do I reset my account password?';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSIsIm5hbWUiOiJKb2huIERvZSJ9.V1Qf4Qk6Zx7Yh2b9lK8e3P0sR2t9uF6a8gHjKlMnOpQ';
  const suggestWithoutSso: ModerationSuggestResponse = await getSearchSuggest(textSearch);
  const suggestWithSso: ModerationSuggestResponse = await getSearchSuggest(textSearch, sso);
  console.log(suggestWithoutSso, suggestWithSso);
})();
[inline-code-end]

---