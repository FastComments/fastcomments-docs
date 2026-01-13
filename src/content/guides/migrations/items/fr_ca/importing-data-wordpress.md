Notre [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) dispose d'un puissant mécanisme d'importation basé sur l'interface utilisateur. Lors de l'installation du plugin,
il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Ceci est fait sans copier ni télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l'interface utilisateur pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas imposer une charge excessive à votre installation WordPress pendant la migration.

### CloudFlare et pare-feu

Pour que la configuration automatisée de WordPress fonctionne, nous devons effectuer des appels à votre installation WordPress.
Des pare-feu comme CloudFlare peuvent nous bloquer et provoquer l'échec de l'intégration. Dans ce cas, [nous pouvons
vous fournir](https://fastcomments.com/auth/my-account/help) un ensemble d'adresses IP à mettre sur liste blanche pour l'intégration.

### Propriété des données

Dans le cadre de notre migration WordPress, toute donnée de commentaire nouvelle ou mise à jour est automatiquement synchronisée vers votre installation WordPress
en coulisses. Cela signifie que, bien que les commentaires soient servis par FastComments lui-même afin de réduire la charge de votre déploiement WordPress,
nous les enregistrons **également** dans votre base de données comme sauvegarde. Cela signifie aussi que si vous désirez quitter FastComments, vos données sont
déjà migrées et à jour.