Dieser Endpunkt ermöglicht das Löschen einer Benutzer-Badge-Zuweisung.

Beispielanfrage:

[inline-code-attrs-start title = 'DELETE Anfrage Beispiel'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Beispielantwort:

[inline-code-attrs-start title = 'Antwort'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Mögliche Fehlerantworten:

[inline-code-attrs-start title = 'Fehler: Fehlende Tenant-ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fehler: Fehlende ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Fehler: Nicht gefunden'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]
