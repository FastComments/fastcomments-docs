Notre [Plugin WordPress](https://wordpress.org/plugins/fastcomments/) dispose d’un puissant mécanisme d’importation basé sur l’interface utilisateur. Lors de l’installation du plugin, il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Cela se fait sans copier ou télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l’interface pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas imposer une charge excessive à votre installation WordPress pendant la migration.

Si vous déplacez votre site hors de WordPress, vous pouvez importer un export XML ou CSV de WordPress au lieu d’utiliser le plugin. Voir
[Déplacer vos commentaires vers un nouveau site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & pare-feux

Pour que la configuration automatisée de WordPress fonctionne, nous devons effectuer des appels à votre installation WordPress. Les pare-feux comme Cloudflare peuvent nous bloquer et entraîner l’échec de l’intégration. Dans ces cas, [nous pouvons vous fournir](https://fastcomments.com/auth/my-account/help) un ensemble d’IP à mettre en liste blanche pour l’intégration.

### Propriété des données

Dans le cadre de notre migration WordPress, toute donnée de commentaire nouvelle ou mise à jour est automatiquement synchronisée avec votre installation WordPress en arrière‑plan. Cela signifie que, bien que les commentaires soient servis par FastComments lui‑même afin de réduire la charge de votre déploiement WordPress, nous **les enregistrons également** dans votre base de données comme sauvegarde. Cela signifie également que si vous souhaitez vous éloigner de FastComments, vos données sont déjà migrées et à jour.