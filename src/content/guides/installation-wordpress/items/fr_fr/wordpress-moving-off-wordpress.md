Si vous migrez votre site hors de WordPress et souhaitez FastComments sur le nouveau site, vous n'avez pas besoin du plugin WordPress. Exportez vos commentaires depuis
WordPress, puis téléversez le fichier sur la [page d'Import](https://fastcomments.com/auth/my-account/manage-data/import) dans le tableau de bord FastComments.

Nous prenons en charge deux formats d'export WordPress.

### WordPress XML (Recommandé)

Il s'agit du fichier de l'exportateur intégré de WordPress, donc aucun plugin supplémentaire n'est nécessaire.

1. Dans votre administration WordPress, allez dans `Tools -> Export`.
2. Sélectionnez `All content` et cliquez sur `Download Export File`.
3. Sur la [page d'Import](https://fastcomments.com/auth/my-account/manage-data/import) de FastComments, sélectionnez `WordPress (.xml)` et téléversez le fichier.

Chaque commentaire est lié à l'URL de l'article sur lequel il a été laissé, qui est déjà présente dans le fichier.

L'import conserve le nom de l'auteur, l'email et le site web, la date, le contenu, l'imbrication des réponses et le fait que le commentaire ait été approuvé. Les avatars des commentateurs sont
récupérés depuis Gravatar. Les votes ne font pas partie de ce format.

### WordPress CSV

Il s'agit du fichier du [plugin WordPress Comments Import & Export de WebToffee](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Installez le plugin dans votre administration WordPress et exportez vos commentaires en CSV.
2. Remplacez chaque valeur `comment_post_ID` par l'URL de l'article.
3. Sur la [page d'Import](https://fastcomments.com/auth/my-account/manage-data/import) de FastComments, sélectionnez `WordPress (.csv)` et téléversez le fichier.

Chaque commentaire est lié à la colonne `comment_post_ID`. WordPress remplit cette colonne avec l'ID de l'article, et votre nouveau site n'a pas d'ID d'articles WordPress,
c'est pourquoi l'étape 2 la remplace par l'URL.

L'import conserve le nom de l'auteur, l'email et le site web, la date, le contenu, l'imbrication des réponses et le fait que le commentaire ait été approuvé. Les avatars des commentateurs sont
récupérés depuis Gravatar. Il conserve également le marqueur de spam de WordPress, ainsi que les likes et dislikes de wpDiscuz lorsque le fichier les inclut.

### Association des commentaires à vos nouvelles pages

Si votre nouveau site conserve les mêmes URL que votre site WordPress, les commentaires apparaissent sur les pages correspondantes sans configuration supplémentaire.

Si le domaine change, exécutez l'[outil de migration de domaine](/guide-migrations.html#migrating-domains) après l'import. Si les URL de certaines pages changent, vous pouvez
[migrer chaque page](/guide-migrations.html#migrating-pages) de son ancienne URL vers la nouvelle.

Pour les migrations de pages en masse, comme la suppression du domaine de la valeur que vous passez au champ [urlId](/guide-customizations-and-configuration.html#url-id)
du widget de commentaires, [ouvrez un ticket de support](https://fastcomments.com/auth/my-account/help) et nous nous en occuperons pour vous.

### Avant de basculer

Vous pouvez exécuter l'import autant de fois que vous le souhaitez. Réimporter le même fichier [ne crée pas de doublons](/guide-migrations.html#importing-data), vous pouvez donc
importer une première fois pour tester le nouveau site, puis importer à nouveau avec vos derniers commentaires juste avant la bascule.

Pour les fichiers d'export de plus de 1 Go, [contactez le support](https://fastcomments.com/auth/my-account/help).

Pour ajouter FastComments à votre nouveau site, consultez le [guide d'Installation](/guide-installation.html).