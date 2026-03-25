## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## レスポンス

戻り値: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## 例

[inline-code-attrs-start title = 'createTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: 'Standard Moderation',
  description: 'Suitable for small-to-medium sites: basic moderation, spam rules, and analytics',
  maxCommentsPerMinute: 50,
  allowAnonymousComments: false, // オプションのパラメータが指定されています
  // オプションのフィールドは省略されています: 例: 高度なモデレーションルール、カスタムCSS
  customConfigParameters: {
    enableProfanityFilter: true,
    imageContentProfanityLevel: 'medium' // 説明用の値。CustomConfigParameters 形式を使用します
  }
};
const response: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
console.log(response);
[inline-code-end]

---