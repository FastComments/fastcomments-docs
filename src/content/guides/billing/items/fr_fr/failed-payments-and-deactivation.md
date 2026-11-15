Un paiement peut échouer parce qu'une carte a expiré, a été remplacée, n'a pas de fonds, ou parce que la banque a bloqué le débit. FastComments vous laisse le temps de corriger la situation avant que quoi que ce soit cesse de fonctionner, et informe les personnes qui peuvent le faire.

### Ce qui se passe

1. **Le débit échoue.** La facture reste `Unpaid` dans [Billing History](https://fastcomments.com/auth/my-account/billing-history) avec un bouton `Pay Now`. Sur un forfait fixe, Stripe peut réessayer le débit. Sur Flex, FastComments réessaie chaque nuit toute facture impayée plus ancienne que la période de grâce.
2. **Un avertissement est envoyé.** Chaque nuit, FastComments vérifie chaque compte. Si une facture Flex reste impayée au-delà de la période de grâce et que la nouvelle tentative échoue, ou si un abonnement à forfait fixe a pris fin parce qu'il n'a pas pu être encaissé, chaque utilisateur disposant de la permission `Billing Admin` reçoit l'e-mail `Your FastComments Tenant Will Be Deactivated Soon!` avec la date, et le compte est programmé pour désactivation une période de grâce plus tard. Le widget continue de fonctionner entre-temps.
3. **Désactivation.** Si rien n'a changé à la date prévue, le compte est désactivé : les commentaires sont masqués sur votre site et les visiteurs voient `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Les administrateurs de facturation reçoivent à nouveau un e-mail. Vos données sont conservées.

La période de grâce est d'une semaine pour la plupart des comptes et de deux semaines pour les comptes de plus d'un an. Sur Flex, elle s'applique deux fois, une fois avant l'avertissement et une fois entre l'avertissement et la désactivation, de sorte qu'il s'écoule au moins deux semaines entre un débit échoué et le masquage des commentaires.

### Comment corriger

Enregistrez une carte valide sur la page [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Cela suffit à lui seul pour réactiver immédiatement le compte et annuler toute désactivation programmée ; la facture impayée est ensuite encaissée avec la nouvelle carte, immédiatement sur un forfait fixe et lors de la vérification nocturne sur Flex. Vous pouvez également payer la facture immédiatement avec `Pay Now` dans Billing History.

Si la carte enregistrée est valide et que l'échec était temporaire, vous n'avez rien à faire : la vérification nocturne constate la facture payée ou l'abonnement actif, annule la désactivation et envoie un e-mail aux administrateurs de facturation pour confirmation. Un compte déjà désactivé est réactivé de la même manière.

### Plusieurs factures à la fois

Si votre carte était invalide pendant quelques mois puis corrigée, chaque mois impayé est encaissé séparément, de sorte que vous pouvez voir plusieurs débits le même jour. Vérifiez la période de facturation de chaque facture dans Billing History : elles correspondent à des mois différents, ce ne sont pas des doublons.

### Vous voyez toujours un avertissement de facturation après avoir payé ?

L'avertissement est lié à un compte spécifique. Si vous avez payé mais qu'un site affiche toujours `billing information is out of date`, le widget de ce site utilise très probablement un compte différent de celui sur lequel vous avez payé. Cela se produit lorsqu'un site a été configuré deux fois, ou lorsqu'il existe un compte sur `fastcomments.com` et un autre sur le déploiement UE à `eu.fastcomments.com`, qui sont facturés séparément. Comparez le `tenantId` dans le code de votre page avec celui sous `Settings -> API/SSO`, ou [contactez le support](https://fastcomments.com/auth/my-account/help) et nous réglerons le problème.