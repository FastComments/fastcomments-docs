[api-resource-header-start name = 'QuestionConfig'; route = 'DELETE /api/v1/question-configs/:id'; creditsCost = 100; api-resource-header-end]

Αυτή η διαδρομή παρέχει την αφαίρεση ενός `QuestionConfig` με βάση το id.

**Αυτό θα διαγράψει όλα τα αντίστοιχα αποτελέσματα ερωτήσεων (αλλά όχι τα σχόλια).** Αυτό αποτελεί μέρος του υψηλού κόστους πιστώσεων.

[inline-code-attrs-start title = 'Αφαίρεση QuestionConfig cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-configs/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
