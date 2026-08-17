Notre [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) dispose d'un puissant mécanisme d'importation basé sur l'interface utilisateur. Lors de l'installation du plugin,
il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Cela se fait sans copier ou télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l'interface utilisateur pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas imposer une charge excessive à votre installation WordPress pendant la migration.

### CloudFlare & FireWalls

Afin que la configuration automatisée de WordPress fonctionne, nous devons effectuer des appels à votre installation WordPress.
Des pare-feu comme Cloudflare peuvent nous bloquer et provoquer l'échec de l'intégration. Dans de tels cas, [nous pouvons vous fournir
vous](https://fastcomments.com/auth/my-account/help) un ensemble d'IP à mettre sur liste blanche pour l'intégration.

### Data Ownership

Dans le cas de notre migration WordPress, toute donnée de commentaire nouvelle ou mise à jour est automatiquement synchronisée avec votre installation WordPress
en arrière-plan. Cela signifie que, bien que les commentaires soient servis par FastComments lui‑même afin de réduire la charge de votre déploiement WordPress,
nous **les enregistrons également** dans votre base de données comme sauvegarde. Cela signifie également que si vous souhaitez vous éloigner de FastComments, vos données sont
déjà migrées et à jour.