Selvom navnet er Domain Migration-værktøjet, kan det også bruges til at migrere kommentarer på tværs af individuelle sider.

Indtast blot de gamle og nye [urlId](/guide-customizations-and-configuration.html#url-id)-værdier i inputfelterne `from` og `to`.

### Migrering af sider via API

For at migrere mange sider på én gang, brug [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Den tager op til 100 `from`- og `to`-par pr. anmodning og kører dem som en batchjob.

Med `type` sat til `urlId` flyttes kun siden, hvis `urlId` er præcis `from`, så `post-1` matcher ikke `post-10`. Dashboard-værktøjet har ikke denne mulighed.

Hver API-migrering kan fortrydes i 90 dage med [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).