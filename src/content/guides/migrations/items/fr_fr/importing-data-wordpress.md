Notre [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) dispose d'un mécanisme d'importation puissant basé sur l'interface utilisateur. Lors de l'installation du plugin, il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Cela se fait sans copier ou télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l'interface pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas imposer une charge excessive à votre installation WordPress pendant la migration.

Si vous déplacez votre site hors de WordPress, vous pouvez importer un export XML ou CSV de WordPress au lieu d'utiliser le plugin. Voir [Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Pour que la configuration automatisée de WordPress fonctionne, nous devons effectuer des appels à votre installation WordPress. Des pare‑feu comme Cloudflare peuvent nous bloquer et provoquer l'échec de l'intégration. Dans ces cas, [nous pouvons vous fournir](https://fastcomments.com/auth/my-account/help) un ensemble d'IP à mettre en liste blanche pour l'intégration.

### Propriété des données

Dans le cadre de notre migration WordPress, toutes les nouvelles données de commentaires ou les données mises à jour sont automatiquement synchronisées en arrière‑plan avec votre installation WordPress. Cela signifie que, bien que les commentaires soient servis par FastComments lui‑même afin de réduire la charge de votre déploiement WordPress, nous **les enregistrons également** dans votre base de données comme sauvegarde. Cela signifie également que si vous souhaitez vous éloigner de FastComments, vos données sont déjà migrées et à jour.