Notre [plugin WordPress](https://wordpress.org/plugins/fastcomments/) dispose d'un puissant mécanisme d'importation basé sur une interface utilisateur. Lors de l'installation du plugin,
il vous guidera pour lier votre installation WordPress à FastComments et copier vos données de commentaires existantes.

**Cela se fait sans copier ni télécharger quoi que ce soit manuellement.**

Le processus de migration vous sera indiqué via l'interface utilisateur pendant la migration. La plupart des migrations ne prennent que quelques minutes.

Le mécanisme est conçu pour ne pas mettre une charge excessive sur votre installation WordPress pendant la migration.

Si vous déplacez votre site hors de WordPress, vous pouvez importer un export WordPress XML ou CSV au lieu d'utiliser le plugin. Voir
[Déplacer vos commentaires vers un nouveau site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare et pare-feux

Pour que la configuration automatisée de WordPress fonctionne, nous devons effectuer des appels vers votre installation WordPress.
Des pare-feux comme Cloudflare peuvent nous bloquer et faire échouer l'intégration. Dans de tels cas, [nous pouvons vous
fournir](https://fastcomments.com/auth/my-account/help) une liste d'IP à mettre en liste blanche pour l'intégration.

### Propriété des données

Dans le cas de notre migration WordPress, toutes les données de commentaires nouvelles ou mises à jour sont automatiquement resynchronisées vers votre installation WordPress
en arrière-plan. Cela signifie que, bien que les commentaires soient servis par FastComments lui-même afin de réduire la charge de votre déploiement WordPress,
nous les enregistrons **également** dans votre base de données en tant que sauvegarde. Cela signifie aussi que si vous souhaitez passer à autre chose que FastComments, vos données sont
déjà migrées et à jour.