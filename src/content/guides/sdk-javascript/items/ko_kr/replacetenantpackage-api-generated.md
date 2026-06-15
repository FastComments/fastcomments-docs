## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'replaceTenantPackage 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-114";
const id: string = "pkg-enterprise-2026-06";
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: "EnterpriseModeration",
  version: "2.4.7",
  enabled: true,
  apiStatus: { state: "active", lastUpdated: "2026-06-10T12:00:00Z" },
  customConfigParameters: { maxCommentLength: 1200, allowImages: true }, // 선택적 설정 포함
  voteStyle: { style: "updown" }
};
const result: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---