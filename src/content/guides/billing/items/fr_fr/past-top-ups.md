Le tableau `Past top-ups` en bas de la page Credits liste vos recharges les plus récentes, de la plus récente à la plus ancienne, avec la `Date`, le `Amount` et le `Status` de chacune. Avant votre première recharge, il affiche `No top-ups yet.`

Chaque recharge a l'un des trois statuts suivants :

- `Credited` : le paiement a réussi et le montant a été ajouté à votre solde.
- `Pending` : le paiement a été initié mais n'est pas terminé, généralement parce qu'une confirmation bancaire n'a pas été finalisée. Les recharges en attente sont vérifiées auprès de Stripe à la prochaine ouverture de la page Credits et passent en `Credited` ou `Failed`.
- `Failed` : la carte a été refusée ou la banque n'a pas approuvé le paiement. Rien n'a été débité et votre solde n'a pas changé.

Le tableau enregistre les recharges effectuées depuis cette page. Les factures payées avec du crédit sont affichées avec les factures elles-mêmes, sous `View Invoices` sur la page [Billing Info](https://fastcomments.com/auth/my-account/billing-info).