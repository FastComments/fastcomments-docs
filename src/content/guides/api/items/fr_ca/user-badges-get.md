Ce point de terminaison vous permet de récupérer les badges des utilisateurs selon divers critères.

Exemple de requête :

[inline-code-attrs-start title = 'Liste des badges utilisateur - Exemple GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Vous pouvez ajouter divers paramètres de requête pour filtrer les résultats :

- `userId` - Obtenir les badges d'un utilisateur spécifique
- `badgeId` - Obtenir les instances d'un badge spécifique
- `type` - Filtrer par type de badge (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, etc. Voir la structure UserBadge pour la liste complète)
- `displayedOnComments` - Filtrer selon si le badge est affiché sur les commentaires (true/false)
- `limit` - Nombre maximal de badges à retourner (par défaut 30, max 200)
- `skip` - Nombre de badges à ignorer (pour la pagination)

Exemple de réponse :

[inline-code-attrs-start title = 'Réponse'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
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
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
    }
  ]
}
[inline-code-end]

Réponses d'erreur possibles :

[inline-code-attrs-start title = 'Erreur : Identifiant du locataire manquant'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Erreur : limite invalide'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]