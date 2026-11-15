Ce qui est facturé sur votre carte, et à quel moment, dépend du type de plan que vous avez.

### Plans fixes

Lorsque vous démarrez un abonnement `Creators` ou `Pro`, le premier mois ou la première année est facturé immédiatement, dans le navigateur, lorsque vous cliquez sur `Pay Now & Start Subscription`. Si votre banque vous demande de confirmer le paiement, effectuez la vérification dans la fenêtre qui s'ouvre. L'abonnement ne démarre qu'une fois le paiement effectué.

Ensuite, l'abonnement se renouvelle le même jour de chaque mois, ou de chaque année si vous avez choisi `Pay Annually`, à compter de la date de souscription. Le renouvellement est facturé automatiquement sur la carte enregistrée et la facture est envoyée par e-mail à votre adresse de facturation. Aucune action n'est requise de votre part.

La facturation annuelle est 15 % moins chère que la mensuelle. Flex ne peut pas être facturé annuellement.

### Flex

Flex est facturé à terme échu : vous utilisez le service au cours d'un mois calendaire, et la facture de ce mois est créée le dernier jour du mois (UTC) et débitée immédiatement sur votre carte. La date de facturation est toujours le dernier jour du mois et ne peut pas être modifiée.

La facture couvre uniquement ce mois calendaire. L'utilisation du mois calendaire durant lequel vous vous êtes inscrit n'est jamais facturée, de sorte que votre première facture arrive au plus tôt à la fin du mois suivant votre inscription.

Si votre utilisation est inférieure au minimum Flex, une ligne indiquant `FastComments Flex. (minimum ... /mo)` complète la facture jusqu'au montant minimum. Le minimum actuel est affiché sur votre page [Billing Info](https://fastcomments.com/auth/my-account/billing-info) et sur la [page des tarifs](https://fastcomments.com/traffic-pricing).

Vous pouvez consulter à tout moment le montant de la prochaine facture. Le `Next Invoice Amount` figure sur la page [Billing Info](https://fastcomments.com/auth/my-account/billing-info), et le détail complet ligne par ligne se trouve sur la page [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Ce qui est comptabilisé est décrit dans [How Flex Usage Is Measured](#how-flex-usage-is-measured).

### L'enregistrement d'une carte ne la débite pas

Une fois votre facturation configurée, l'enregistrement ou la mise à jour d'une carte sans changement de plan ne la débite jamais, sauf si vous avez une facture impayée. Le formulaire indique `You won't be charged now unless you change plans.` et la carte est simplement stockée chez Stripe pour la prochaine facture. Ce n'est que si vous n'avez pas encore de facturation valide que le formulaire indique `Your card will be charged now.`, et même dans ce cas, un compte Flex n'est pas débité avant la fin du mois. Tout [crédit](#credits) prépayé sur votre compte est appliqué à chaque facture avant que la carte ne soit débitée.

### Sous-comptes gérés

Si votre compte gère des sous-comptes (par exemple un par site ou environnement, ou des clients en marque blanche), seul le compte parent est facturé. L'utilisation de chaque sous-compte apparaît sous forme de lignes distinctes sur la facture Flex du compte parent, préfixées par le nom du sous-compte, et les sous-comptes partagent le moyen de paiement et le statut de facturation du compte parent.