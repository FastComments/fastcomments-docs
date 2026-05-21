### Installer depuis la boutique d'applications Shopify

1. Ouvrez la [fiche FastComments sur la boutique d'applications Shopify](https://apps.shopify.com/fastcomments).
2. Cliquez sur **Add app** et choisissez le forfait que vous souhaitez pendant le flux d'installation.
3. Shopify vous redirige vers l'administration FastComments intégrée à Shopify lorsque l'installation est terminée.

C'est toute l'installation. Il n'y a rien à coller dans les fichiers de votre thème.

### Ce qui est configuré pour vous

L'installation exécute tout ce que vous feriez autrement manuellement :

- Un tenant FastComments est créé pour votre boutique et lié à votre domaine de boutique.
- L'URL de la boutique est ajoutée aux domaines autorisés du tenant, afin que les commentaires se chargent sans erreur de domaine.
- Un metafield de boutique `fastcomments.tenant_id` est écrit afin que chaque bloc sache contre quel tenant s'afficher.
- L'authentification unique pour vos clients Shopify est activée par défaut.
- La facturation passe par Shopify Managed Pricing. Les frais apparaissent sur votre facture Shopify habituelle. Mettez à niveau, rétrogradez ou annulez depuis **Paramètres > Applications et canaux de vente > FastComments** dans votre administration Shopify.

Si votre boutique était déjà cliente de FastComments avant que vous n'installiez l'application, l'installation réutilise le tenant existant au lieu d'en créer un nouveau.

### L'interface d'administration intégrée

Lorsque vous ouvrez l'application FastComments depuis votre administration Shopify, vous arrivez sur un tableau de bord avec des vignettes d'un clic vers l'interface complète de FastComments :

- **Dashboard** : paramètres de compte, utilisation et détails de l'abonnement.
- **Moderation Queue** : approuver, rejeter et répondre aux commentaires dans toute votre boutique.
- **Customize** : ajuster les couleurs du widget, polices, règles de modération et configuration.
- **Ratings & Reviews Helper** : configurer les évaluations par étoiles et les questions d'avis si vous souhaitez utiliser le bloc Reviews Summary.

Chaque tuile ouvre FastComments avec un lien de connexion à usage unique, vous n'avez donc pas besoin d'une connexion séparée.

### Suivant : ajoutez des blocs à votre boutique

Ouvrez l'éditeur de thème Shopify (**Boutique en ligne > Thèmes > Personnaliser**), ouvrez le gabarit sur lequel vous souhaitez ajouter des commentaires ou des avis, et cliquez sur **Add block**. Les blocs FastComments apparaissent sous **Applications**. Le reste de ce guide couvre chacun d'entre eux.