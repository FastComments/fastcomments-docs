Dette endpoint giver dig mulighed for at hente en brugers badge-fremskridtspost efter deres bruger-ID.

Eksempel på Anmodning:

[inline-code-attrs-start title = 'GET Anmodningseksempel'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/user/user456?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Eksempel på Svar:

[inline-code-attrs-start title = 'Svar'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
    "id": "progress123",
    "tenantId": "tenant001",
    "userId": "user456",
    "firstCommentId": "comment789",
    "firstCommentDate": 1650532511000,
    "autoTrustFactor": 0.75,
    "progress": {
      "0": 42,
      "1": 120,
      "2": 15,
      "3": 3,
      "5": 5,
      "6": 1800000,
      "8": 0,
      "7": 0
    }
  }
}
[inline-code-end]

Mulige Fejlsvar:

[inline-code-attrs-start title = 'Fejl: Manglende Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fejl: Manglende Bruger ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "The User ID (path param: userId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fejl: Ikke Fundet'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "No badge progress found for user user456 in tenant tenant001."
}
[inline-code-end]
