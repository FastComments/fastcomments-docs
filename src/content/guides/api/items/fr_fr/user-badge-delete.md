Ce endpoint vous permet de supprimer une attribution de badge utilisateur.

Exemple de Requête :

[inline-code-attrs-start title = 'Exemple de Requête DELETE'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Exemple de Réponse :

[inline-code-attrs-start title = 'Réponse'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Réponses d'Erreur Possibles :

[inline-code-attrs-start title = 'Erreur: ID de Locataire Manquant'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur: ID Manquant'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur: Non Trouvé'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]
