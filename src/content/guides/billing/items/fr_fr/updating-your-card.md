Il n'existe pas d'écran séparé pour « modifier la carte ». Pour remplacer la carte enregistrée, ouvrez [Informations de facturation](https://fastcomments.com/auth/my-account/billing-info), laissez votre plan tel quel, saisissez les nouvelles informations de carte sous `Payment method`, puis cliquez sur `Save Info`.

La nouvelle carte devient celle par défaut pour toutes les factures futures. Rien n'est débité lors de l'enregistrement, sauf si vous avez une facture impayée, auquel cas cette facture est encaissée avec la nouvelle carte : immédiatement sur un plan fixe, et du jour au lendemain sur Flex. Tant que votre facturation est en règle, le formulaire indique `You won't be charged now unless you change plans.` Si votre abonnement au plan fixe est terminé, l'enregistrement d'une carte en démarre un nouveau et la première période est facturée, comme lors de la première inscription.

Si votre banque exige une confirmation de la carte, une fenêtre de confirmation s'ouvre après avoir cliqué sur enregistrer. La carte n'est stockée qu'une fois approuvée par la banque.

### Qui peut faire cela

Seul le `Account Owner` ou un utilisateur disposant de la permission `Billing Admin` peut enregistrer une carte ou modifier le plan. Les autres utilisateurs voient `Only the account owner or a billing admin can change the plan or payment method.` Si le menu `Settings` est grisé ou si la page de facturation vous redirige vers la page d'accueil, vous êtes connecté en tant que commentateur ou utilisateur sans accès à la facturation plutôt qu'en tant que propriétaire du compte. Connectez-vous avec le compte qui a créé le site, ou demandez au propriétaire de vous accorder la permission ; voir [Qui peut gérer la facturation](#who-can-manage-billing).

### Cartes refusées ou bloquées

Si la carte est refusée, la page affiche `Your card was declined. Check that the entered information is correct.` et rien n'est enregistré. Vérifiez le numéro, la date d'expiration, le CVC et l'adresse de facturation, puis réessayez. Si la banque a refusé une confirmation, vous voyez `Your bank did not approve the payment. Try again, or use a different card.`

Certaines banques bloquent les paiements en ligne récurrents, ou bloquent une carte après une tentative refusée. Dans ce cas, essayez une autre carte, ou payez une facture ouverte avec `Pay Now` dans [Historique de facturation](https://fastcomments.com/auth/my-account/billing-history). Si votre banque n'autorise pas du tout les prélèvements automatiques, [contactez le support](https://fastcomments.com/auth/my-account/help) et nous pourrons envoyer un lien de paiement pour la facture à la place.

Seules les cartes sont acceptées. La liste `Country` du formulaire contient les pays que Stripe peut facturer ; si le vôtre est absent, contactez le support.