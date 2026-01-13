[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιτρέπει την αναζήτηση για τη χρήση ενός ενοικιαστή ανά έτος, μήνα και ημέρα. Μπορούν να επιστραφούν έως 365 αντικείμενα, και το κόστος είναι 1 πίστωση api ανά 10 αντικείμενα.

Τα αντικείμενα απάντησης ταξινομούνται κατά την ημερομηνία δημιουργίας τους (τα παλαιότερα πρώτα).

[inline-code-attrs-start title = 'Αναζήτηση TenantDailyUsage cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Based on UTC. **/
    yearNumber?: number
    /** Zero-based. Based on UTC. **/
    monthNumber?: number
    /** One-based. Based on UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]
