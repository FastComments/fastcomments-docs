## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b1c';
const id: string = 'pkg_pro_2026';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  planCode: 'pro_annual',
  seats: 12,
  expiresAt: '2027-01-01T00:00:00Z',
  autoRenew: true, // דגל אופציונלי המדגים פרמטר אופציונלי
  notes: 'Upgrade for team collaboration'
};
const result: APIEmptyResponse = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---