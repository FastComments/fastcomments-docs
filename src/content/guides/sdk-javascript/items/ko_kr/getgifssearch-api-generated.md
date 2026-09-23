## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| search | string | 예 |  |
| locale | string | 아니오 |  |
| rating | string | 아니오 |  |
| page | number | 아니오 |  |

## 응답

반환: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## 예시

[inline-code-attrs-start title = 'getGifsSearch 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const search: string = "celebration fireworks";
  const locale: string = "fr-FR";
  const rating: string = "g";
  const page: number = 1;

  const fullResult: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  const minimalResult: GetGifsSearchResponse = await getGifsSearch(tenantId, "dog memes");
}

runExample();
[inline-code-end]

---