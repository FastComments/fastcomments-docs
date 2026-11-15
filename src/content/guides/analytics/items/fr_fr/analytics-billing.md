Toutes les métriques pour lesquelles vous pouvez être limité en débit ou facturé sont présentées sur la [Page d'analyse de facturation](https://fastcomments.com/auth/my-account/analytics/billing).

Voici quelques exemples de métriques suivies :

- Chargements de pages (pour le mois en cours)
- Crédits API (pour le mois en cours)
- Nouveaux commentaires stockés (pour le mois en cours)
- Utilisateurs du tenant
- Utilisateurs SSO (utilisateurs SSO réguliers sans permissions d'administrateur/modérateur)
- Administrateurs SSO (utilisateurs SSO avec les indicateurs `isAccountOwner` ou `isAdminAdmin` - facturés séparément des utilisateurs SSO réguliers)
- Modérateurs SSO (utilisateurs SSO avec l'indicateur `isCommentModeratorAdmin` - facturés séparément des utilisateurs SSO réguliers)
- Modérateurs
- Domaines

Pour les plans `Creators` et `Pro`, ces métriques servent simplement de repère pour savoir si vous êtes proche des limites de votre compte ou si vous les avez atteintes.

Pour le plan `Flex`, cette page affiche une ventilation des coûts de l'utilisation actuelle pour la **prochaine** facture. Cela permet de comprendre facilement comment vous serez facturé avant que cela ne se produise. Vous pouvez consulter les factures passées dans [Historique de facturation](https://fastcomments.com/auth/my-account/billing-history). La façon dont chaque ligne est comptabilisée, ainsi que le moment où la facture est débitée, sont expliqués dans le [Guide de facturation](/guide-billing.html#how-flex-usage-is-measured).

Les métriques `Page Loads`, `API Credits` et `New Comments Stored` peuvent avoir jusqu'à une minute de retard. Les autres métriques sont en temps réel.

Ces informations sont stockées sur une base mensuelle pendant un an, ou jusqu'à ce que vous supprimiez votre tenant, à des fins historiques, mais l'interface n'affichera que la période en cours.