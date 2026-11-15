Sur Flex, les personnes pouvant administrer ou modérer votre compte sont facturées comme des sièges mensuels. C'est la partie d'une facture Flex qui surprend le plus souvent, généralement parce que des sièges ont été créés sans que personne ne s'en aperçoive.

### Ce Qui Compte Comme Un Siège

- **Admins** : les utilisateurs sur la page `Users` de votre compte ayant une permission administrative (facturation, personnalisation, analytique, modération, etc.). Le premier admin, normalement le propriétaire du compte, est gratuit ; chaque admin supplémentaire est un siège.
- **Modérateurs** : les personnes sur la page `Moderators` qui ont accepté leur invitation. Une invitation en attente n'est pas facturée. Un utilisateur dont la seule permission est la modération de commentaires est facturé comme modérateur, et non comme admin.
- **SSO Admins** et **SSO Moderators** : les utilisateurs SSO avec lesquels votre site se connecte avec `isAccountOwner`, `isAdminAdmin`, ou `isCommentModeratorAdmin` défini dans le payload SSO. Ils sont facturés aux mêmes tarifs que les admins et modérateurs réguliers. Voir [SSO User Structure](/guide-api.html#sso-user-structure).

Les sièges sont dédupliqués par adresse e-mail, donc une personne qui est à la fois admin et modératrice, ou qui a à la fois un utilisateur SSO et une connexion au tableau de bord avec le même e-mail, est facturée une seule fois, au tarif le plus élevé. Une personne avec deux adresses e-mail différentes compte pour deux sièges.

Sur les comptes facturés par utilisateurs actifs mensuels, un modérateur SSO qui ne s'est pas connecté durant le mois n'est pas facturé pour ce mois.

### Des Sièges Que Vous N'avez Pas Ajoutés

Deux fonctionnalités créent des sièges automatiquement :

- **Synchronisation des admins SSO.** Lorsqu'un utilisateur SSO se connecte avec un e-mail correspondant à l'un de vos utilisateurs du tableau de bord, les permissions d'admin ou de modérateur de cet utilisateur sont copiées vers l'utilisateur SSO. Avec le SSO WordPress, cela signifie que chaque administrateur WordPress peut devenir un admin ou modérateur FastComments. Désactivez cela sous `Customize` avec l'option `Disable Automatic Admin Migration` sur votre règle de personnalisation.
- **E-mails différents pour la même personne.** Si votre connexion au tableau de bord utilise un e-mail et votre utilisateur SSO un autre, cela fait deux sièges. Utiliser le même e-mail sur les deux les fusionne.

### Vérifier Et Supprimer Des Sièges

La page [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) liste vos nombres de `Moderators`, `SSO Admins` et `SSO Moderators`, et l'aperçu de la prochaine facture montre ce qu'ils coûtent. Supprimez les personnes dont vous n'avez pas besoin depuis les pages `Users` et `Moderators`, ou retirez les indicateurs d'admin de votre payload SSO. Les sièges sont comptabilisés lors de la création de la facture à la fin du mois, donc un siège supprimé avant n'est pas facturé pour ce mois-là.

Voir [Adding Moderators](/guide-users.html#adding-moderators) et [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) pour savoir comment gérer les deux.