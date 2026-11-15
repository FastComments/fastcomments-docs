Les recharges sont facturées sur le moyen de paiement enregistré sur votre compte. Si vous n'avez pas encore enregistré de carte, la page Crédits affiche `Add a payment method first` avec un bouton `Go to billing`. Enregistrez une carte sur la page [Billing Info](https://fastcomments.com/auth/my-account/billing-info), puis revenez.

Pour ajouter du crédit :

1. Ouvrez [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) et cliquez sur `Manage Credits`.
2. Sous `Add credit`, choisissez l'un des montants prédéfinis, ou sélectionnez `Other amount` et saisissez le vôtre.
3. Cliquez sur le bouton `Add`. Il affiche le montant choisi, par exemple `Add $25.00`.
4. Si votre banque vous demande de confirmer le paiement, effectuez la vérification dans la fenêtre qui s'ouvre.

Une fois le paiement effectué, la page affiche `Credit added` avec votre nouveau solde, et la recharge apparaît dans `Past top-ups` comme `Credited`. Cliquez sur `Add more` pour recharger à nouveau.

### Montants et limites

Les préréglages sont `10`, `25`, `50` et `100` dans votre devise de facturation. Un montant personnalisé peut aller de `5` à `10 000` par recharge, et les limites exactes pour votre devise sont affichées sous le formulaire. Pour les devises sans décimales, comme le yen japonais, les préréglages et les limites sont cent fois plus élevés afin de représenter à peu près la même somme.

Les recharges sont toujours dans la devise de facturation de votre compte, la même que celle de vos factures. La page vous indique quelle devise sera débitée.

### Confirmation bancaire (3D Secure)

Certaines banques exigent la confirmation d'un paiement en ligne, généralement via votre application bancaire ou un code à usage unique. Si c'est le cas de la vôtre, une fenêtre de confirmation s'ouvre après avoir cliqué sur `Add`. Votre carte n'est débitée, et votre solde mis à jour, qu'après l'approbation de la banque.

Si vous fermez la page avant de terminer la confirmation, la recharge est enregistrée comme `Pending`. La prochaine fois que vous ouvrez la page Crédits, nous vérifions le statut du paiement auprès de Stripe et le marquons `Credited` ou `Failed` en conséquence. Si la banque n'approuve pas le paiement, la page affiche `Your bank did not approve the payment. Try again, or use a different card.` et rien n'est débité.

### Cartes refusées

Si votre carte est refusée, la page affiche `Your card was declined. Check your card on the billing page, or try another card.` Rien n'est débité, votre solde ne change pas, et la tentative est listée dans `Past top-ups` comme `Failed`. Mettez à jour la carte sur la page [Billing Info](https://fastcomments.com/auth/my-account/billing-info) et réessayez.

Pour protéger votre carte, un compte peut effectuer un nombre limité de tentatives de recharge par heure. Si vous voyez `Too many top-up attempts. Wait a while and try again.`, attendez et réessayez plus tard.

Les paiements sont traités par Stripe. FastComments ne stocke jamais les détails de votre carte.