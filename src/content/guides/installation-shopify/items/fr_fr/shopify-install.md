### Installer depuis le Shopify App Store

1. Ouvrez la [liste FastComments sur le Shopify App Store](https://apps.shopify.com/fastcomments).
2. Cliquez sur **Add app** et choisissez le plan souhaité pendant le processus d'installation.
3. Shopify vous redirige vers l'administration FastComments dans Shopify une fois l'installation terminée.

C’est l’installation complète. Il n’y a rien à coller dans vos fichiers de thème.

### Ce qui est configuré pour vous

L’installation exécute tout ce que vous feriez manuellement :

- Un locataire FastComments est créé pour votre boutique et lié à votre domaine de boutique.
- L’URL de votre boutique est ajoutée aux domaines autorisés du locataire, de sorte que les commentaires se chargent sans erreur de domaine.
- Un champ méta de boutique `fastcomments.tenant_id` est écrit afin que chaque bloc sache quel locataire utiliser.
- L’authentification unique (SSO) pour vos clients Shopify est activée par défaut.
- La facturation passe par Shopify Managed Pricing. Les frais apparaissent sur votre facture Shopify habituelle. Mettez à niveau, rétrogradez ou annulez depuis **Settings > Apps and sales channels > FastComments** dans votre admin Shopify.

Si votre boutique était déjà cliente FastComments avant d’installer l’application, l’installation réutilise le locataire existant au lieu d’en créer un nouveau.

### L’administration intégrée

Lorsque vous ouvrez l’application FastComments depuis votre admin Shopify, vous accédez à un tableau de bord avec des tuiles en un clic vers le backend complet de FastComments :

- **Dashboard** : paramètres du compte, utilisation et détails de l’abonnement.
- **Moderation Queue** : approuver, rejeter et répondre aux commentaires dans toute votre boutique.
- **Customize** : ajuster les couleurs du widget, les polices, les règles de modération et la configuration.
- **Ratings & Reviews Helper** : configurer les évaluations par étoiles et les questions d’avis si vous souhaitez utiliser le bloc Résumé des avis.

Chaque tuile ouvre FastComments avec un lien de connexion à usage unique, vous n’avez donc pas besoin d’une connexion séparée.

### Prochaine étape : ajouter des blocs à votre boutique

Ouvrez l’éditeur de thème Shopify (**Online Store > Themes > Customize**), ouvrez le modèle auquel vous souhaitez ajouter des commentaires ou des avis, puis cliquez sur **Add block**. Les blocs FastComments apparaissent sous **Apps**. Le reste de ce guide couvre chacun d’eux.