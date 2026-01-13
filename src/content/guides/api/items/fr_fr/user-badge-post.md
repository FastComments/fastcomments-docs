Ce endpoint vous permet de créer une nouvelle attribution de badge utilisateur.

Exemple de Requête :

[inline-code-attrs-start title = 'Exemple de Requête POST'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

Le corps de la requête doit contenir les paramètres suivants :

- `userId` (requis) - L'ID de l'utilisateur auquel attribuer le badge
- `badgeId` (requis) - L'ID du badge à attribuer
- `displayedOnComments` (optionnel) - Si le badge doit être affiché sur les commentaires de l'utilisateur (par défaut à true)

Notes Importantes :
1. Le badge doit exister et être activé dans le catalogue de badges de votre locataire
2. Vous ne pouvez attribuer des badges qu'aux utilisateurs qui appartiennent à votre locataire ou qui ont commenté sur votre site

Exemple de Réponse :

[inline-code-attrs-start title = 'Réponse'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
    "id": "badge123",
    "userId": "user456",
    "badgeId": "badgeDef789",
    "fromTenantId": "tenant001",
    "createdAt": 1650532511000,
    "receivedAt": 1650532511000,
    "type": 14,
    "name": "Special Contributor",
    "description": "Awarded to special contributors to our community",
    "displayLabel": "Special",
    "backgroundColor": "#4a5568",
    "textColor": "#ffffff",
    "displayedOnComments": true,
    "order": 1
  }
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

[inline-code-attrs-start title = 'Erreur: ID Utilisateur Manquant'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur: ID de Badge Manquant'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur: Badge Non Trouvé'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur: Utilisateur Non Autorisé'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]
