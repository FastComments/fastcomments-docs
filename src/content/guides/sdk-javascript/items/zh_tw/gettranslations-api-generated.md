## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| namespace | string | 是 |  |
| component | string | 是 |  |
| locale | string | 否 |  |
| useFullTranslationIds | boolean | 否 |  |

## 回應

返回：[`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTranslationsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTranslations() {
  const adminDashboard: GetTranslationsResponse = await getTranslations('admin', 'dashboard');
  const userProfileFr: GetTranslationsResponse = await getTranslations('user', 'profile', 'fr-FR', true);
}
[inline-code-end]