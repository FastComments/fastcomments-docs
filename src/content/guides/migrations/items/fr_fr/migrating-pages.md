Bien que son nom soit l'outil de migration de domaine, il peut également être utilisé pour migrer des commentaires entre des pages individuelles.

Saisissez simplement les anciennes et nouvelles valeurs [urlId](/guide-customizations-and-configuration.html#url-id) dans les champs de saisie `from` et `to`.

### Migration de pages via l'API

Pour migrer plusieurs pages à la fois, utilisez [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Elle accepte jusqu'à 100 paires `from` et `to` par requête et les exécute en tant que tâche par lots.

Avec `type` défini sur `urlId`, seule la page dont le `urlId` correspond exactement à `from` est déplacée, donc `post-1` ne correspond pas à `post-10`. L'outil du tableau de bord ne dispose pas de cette option.

Chaque migration via l'API peut être annulée pendant 90 jours avec [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).