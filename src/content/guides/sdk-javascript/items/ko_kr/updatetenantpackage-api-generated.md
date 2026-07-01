## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 예 |  |

## 응답

반환: [`UpdateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantPackageResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateTenantPackage 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9876";
const packageId: string = "pkg-2023";

const customConfig: CustomConfigParameters = {
  enableSpamFilter: true,
  spamRatingThreshold: 4,
};

const updateBody: UpdateTenantPackageBody = {
  displayName: "Enterprise Pro",
  customConfig,
};

const response: UpdateTenantPackageResponse = await updateTenantPackage(
  tenantId,
  packageId,
  updateBody
);
[inline-code-end]