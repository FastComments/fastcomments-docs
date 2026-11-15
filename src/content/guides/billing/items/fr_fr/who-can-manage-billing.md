La facturation est réservée à deux types d'utilisateurs :

- Le `Account Owner`, l'utilisateur qui a créé le compte.
- Tout utilisateur disposant de la permission `Billing Admin`.

Ce sont les seuls utilisateurs qui peuvent changer de plan, enregistrer ou remplacer une carte, ouvrir les `Invoice Settings`, définir les limites de dépenses Flex et ajouter du crédit. Tout autre utilisateur qui tente de le faire voit le message `Only the account owner or a billing admin can change the plan or payment method.`

Les e-mails de facturation, tels que le rappel de mi-période d'essai, les avertissements de paiement échoué et les avis de désactivation, sont envoyés aux utilisateurs disposant de la permission `Billing Admin`. Accordez cette permission à toute personne qui devrait les recevoir, par exemple quelqu'un du service financier, et elle recevra les e-mails sans avoir besoin d'un autre accès au tableau de bord. Les rappels envoyés après la fin d'un essai sans informations de facturation sont envoyés à tous les utilisateurs du tableau de bord du compte. Chacun de ces e-mails contient un lien de désabonnement ; un utilisateur qui se désabonne des notifications du compte cesse de les recevoir.

Les factures elles-mêmes sont envoyées à la `Billing Email` dans les [Invoice Settings](#invoice-settings), qui n'a pas besoin d'appartenir à un utilisateur du tableau de bord.

La page [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) nécessite plutôt la permission `Analytics Admin`, car elle affiche l'utilisation plutôt que les détails de paiement.

Pour accorder ou retirer la permission, ouvrez la page `Users` et modifiez l'utilisateur. Voir [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Avec Flex, n'oubliez pas que chaque administrateur supplémentaire est un siège facturé, voir [Admin and Moderator Seats](#admin-and-moderator-seats).

Si vous êtes connecté et que vous ne voyez pas du tout `Billing Info`, ou si vous êtes renvoyé vers la page d'accueil, vous êtes connecté avec un compte de commentateur plutôt qu'avec un utilisateur du tableau de bord. Déconnectez-vous et connectez-vous avec le compte qui possède le site.