Le bloc **FastComments** prend en charge l'authentification unique (SSO) afin que vos clients Shopify puissent commenter en leur nom sans créer un compte FastComments séparé.

### Comment cela fonctionne

Lorsque un visiteur connecté à votre boutique ouvre une page contenant le bloc **FastComments** :

1. Le bloc détecte l'objet Shopify `customer`.
2. Il envoie le nom et l'email du client à FastComments via une requête d'app proxy signée.
3. FastComments crée ou associe un utilisateur identifié par `shopify-{customerId}`, de sorte que le même client corresponde toujours au même utilisateur FastComments entre les sessions et les réinstallations.
4. Le nom du visiteur apparaît sur ses commentaires. Il n'est pas invité à se connecter de nouveau.

Si le visiteur n'est pas connecté à la boutique, le bloc revient au commentaire anonyme (ou au flux de connexion FastComments, selon la configuration de votre widget).

### Désactiver le SSO

Le SSO est activé par défaut pour chaque bloc **FastComments**. Pour le désactiver sur un bloc spécifique :

1. Ouvrez l'éditeur de thème Shopify.
2. Ouvrez le modèle qui contient le bloc et cliquez sur le bloc pour le sélectionner.
3. Décochez **SSO**.
4. Cliquez sur **Enregistrer**.

Désactivez le SSO si vous souhaitez que les commentateurs choisissent une identité distincte pour la conversation. Par exemple, une page communautaire interne où le personnel commente sous un autre nom d'affichage.

### Ce que FastComments reçoit

La charge utile SSO envoyée pour chaque client contient :

- Un identifiant d'utilisateur dérivé de l'ID client Shopify (`shopify-{customerId}`).
- L'email du client (utilisé pour identifier l'utilisateur ; non affiché publiquement).
- Le nom d'affichage du client (utilisé comme nom d'auteur de son commentaire).

Aucun historique de commandes, donnée de paiement ou adresse n'est envoyé. La charge utile est signée côté serveur ; le navigateur du client ne voit jamais de justificatif d'identité.

### Liens de connexion et de déconnexion

Lorsque le SSO est activé, les liens de connexion et de déconnexion du widget de commentaires pointent vers `/account/login` et `/account/logout`, les routes standard de compte client Shopify. Il n'y a rien à configurer. Les liens fonctionnent pour toute boutique ayant les comptes clients activés.