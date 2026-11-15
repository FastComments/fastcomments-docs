Chaque facturation, qu'il s'agisse d'un renouvellement de plan, d'un mois Flex ou d'un changement de plan, produit une facture. Les factures servent également de reçus une fois payées.

### Trouver vos factures

Ouvrez [Billing History](https://fastcomments.com/auth/my-account/billing-history), ou cliquez sur `View Invoices` sur la page Billing Info. Le tableau liste chaque facture avec sa date, sa période de facturation, son numéro de facture, sa description, son total, son statut (`Paid`, `Unpaid`, `Draft` ou `Void`) et sa date d'échéance, dix par page, les plus récentes en premier.

Pour chaque facture :

- `View` ouvre la facture sur la page hébergée par Stripe, où vous pouvez également la télécharger.
- `PDF` télécharge directement le PDF.
- `Pay Now` apparaît à la place de `View` sur une facture impayée et vous dirige vers la page de paiement Stripe de cette facture, où vous pouvez la payer directement.

Les factures sont également envoyées par e-mail à votre adresse de facturation lors de leur création. Pour qu'elles soient envoyées à une boîte mail financière plutôt qu'au propriétaire du compte, définissez le `Billing Email` dans [Invoice Settings](#invoice-settings).

### Contenu de la facture

La facture affiche le nom et l'adresse de facturation ainsi que le numéro de TVA / identifiant fiscal provenant de [Invoice Settings](#invoice-settings), les lignes de détail, tout [crédit](#credits) prépayé qui a été appliqué, et le montant facturé sur votre carte. Les factures Flex listent une ligne par type d'utilisation, voir [How Flex Usage Is Measured](#how-flex-usage-is-measured). Pour les adresses de facturation situées dans l'Union européenne, les factures Flex sont émises avec 0 % de TVA et le pied de page `Reverse Charge Transaction: Customer to pay VAT`.

Les modifications des paramètres de facture s'appliquent aux factures créées après leur enregistrement. Une facture déjà émise ne peut pas être modifiée, donc définissez les informations de votre société et votre numéro de TVA / identifiant fiscal avant la prochaine date de facturation.

### Comptes facturés ailleurs

Si votre facturation est gérée en externe, par exemple par une facturation organisée avec le support, les factures vous sont envoyées pour paiement plutôt que débitées d'une carte et peuvent ne pas apparaître dans Billing History. Les comptes installés via Shopify ou BigCommerce sont facturés par cette plateforme, et la page Billing Info renvoie vers la page du plan correspondante. Les sous-comptes facturés via un compte parent n'ont pas de factures propres ; voir [When You Are Charged](#when-you-are-charged).