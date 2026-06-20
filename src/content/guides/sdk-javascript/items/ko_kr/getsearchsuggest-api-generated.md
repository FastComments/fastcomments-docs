## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSearchSuggest 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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