Notre [plugin WordPress](https://wordpress.org/plugins/fastcomments/) dispose d'un puissant mécanisme d'importation basé sur une interface utilisateur. Lors de l'installation du plugin, il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Cela se fait sans copier ni télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l'interface utilisateur pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas imposer une charge excessive à votre installation WordPress pendant la migration.

### CloudFlare et pare-feu

Pour que la configuration WordPress automatisée fonctionne, nous devons effectuer des appels vers votre installation WordPress. Des pare-feu comme Cloudflare peuvent nous bloquer et provoquer l'échec de l'intégration. Dans de tels cas, [nous pouvons vous fournir](https://fastcomments.com/auth/my-account/help) un ensemble d'adresses IP à mettre sur liste blanche pour l'intégration.

### Propriété des données

Dans le cas de notre migration WordPress, toutes les nouvelles données de commentaires ou les commentaires mis à jour sont automatiquement synchronisés en arrière-plan vers votre installation WordPress. Cela signifie que, bien que les commentaires soient servis par FastComments lui-même pour alléger la charge de votre déploiement WordPress, nous les enregistrons **également** dans votre base de données comme sauvegarde. Cela signifie aussi que si vous souhaitez quitter FastComments, vos données sont déjà migrées et à jour.