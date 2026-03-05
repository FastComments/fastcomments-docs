Lorsque les courriels envoyés par FastComments rebondissent ou sont signalés comme indésirables par le destinataire, le fournisseur de messagerie ajoute cette adresse à une liste de suppression. FastComments synchronise ces listes de suppression quotidiennement afin qu'aucun autre courriel ne soit envoyé aux adresses qui ne peuvent pas les recevoir.

Les utilisateurs et les modérateurs dont l'adresse courriel est en suppression ne recevront aucune notification par courriel, y compris les notifications de réponse, les notifications de mention, les alertes d'administration et les courriels récapitulatifs. Un insigne rouge "Courriel supprimé" apparaîtra à côté des utilisateurs et modérateurs concernés dans l'interface d'administration.

#### Viewing Suppressed Emails

Les administrateurs du locataire disposant de l'autorisation Gérer les données peuvent consulter les courriels supprimés sur la page [Courriels supprimés](https://fastcomments.com/auth/my-account/suppressed-emails), sous Gérer les données.

La page affiche un tableau de toutes les adresses courriel en suppression associées aux utilisateurs, modérateurs et commentateurs de votre locataire. Vous pouvez filtrer par adresse courriel à l'aide du champ de recherche.

#### Removing a Suppression

Pour retirer une suppression, cliquez sur le bouton **Supprimer** à côté de l'entrée dans le tableau. Vous serez dirigé vers une page de confirmation affichant les détails de la suppression. Cliquez sur **Confirmer la suppression** pour continuer.

Lorsqu'une suppression est retirée, FastComments contacte le fournisseur de messagerie pour débloquer l'adresse et supprime le drapeau de suppression de tous les enregistrements d'utilisateurs et de modérateurs associés.

#### Rate Limits

- Chaque adresse courriel ne peut être retirée de la liste de suppression qu'une fois tous les 30 jours.
- Chaque locataire peut effectuer jusqu'à 5 retraits par mois civil.

Si une suppression réapparaît après son retrait, cela signifie que l'adresse courriel a de nouveau rebondi ou a été signalée comme indésirable. Dans ce cas, le problème de délivrabilité sous-jacent doit être résolu avant de tenter un nouveau retrait.