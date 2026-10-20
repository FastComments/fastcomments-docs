FastComments fournit un moyen automatisé pour vous de migrer vos commentaires entre domaines.

La migration de domaine nécessite simplement un domaine `from` et un domaine `to`.

Cela **déplace** les commentaires, cela ne les copie pas. Si vous souhaitez copier les commentaires, contactez‑nous.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Outil de migration de domaine avec les champs de domaine from et to et le nombre de commentaires migrés'; title='Migration de domaines' app-screenshot-end]

Ceci est également utile, par exemple, si une partie de votre migration vers FastComments implique une migration depuis un autre fournisseur, de sorte que vos données d'importation de commentaires peuvent contenir des données qui doivent être migrées. Dans ce cas, vous pouvez exécuter l'importation, puis la migration de domaine.

### Suivi de la progression

L'outil de migration de domaine utilise le même système de traitement des tâches FastComments que les autres outils de gestion de données.

Il peut y avoir un délai avant le démarrage de votre migration. C'est normal, car le système vérifie périodiquement les nouvelles tâches à traiter.

Au fur et à mesure que la tâche s'exécute, elle affichera le nombre de commentaires trouvés à migrer, ainsi que le nombre déjà migrés.

### Migration de domaines via API

Les domaines peuvent également être migrés avec [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), en utilisant `type` défini sur `domain`. La progression est disponible via [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), et la migration peut être annulée pendant 90 jours.

---