Lorsque les e-mails envoyés par FastComments rebondissent ou sont marqués comme spam par le destinataire, le fournisseur d'e-mails ajoute cette adresse à une liste de suppression. FastComments synchronise ces listes de suppression quotidiennement afin qu'aucun e-mail supplémentaire ne soit envoyé aux adresses qui ne peuvent pas les recevoir.

Les utilisateurs et modérateurs dont les adresses e-mail sont supprimées ne recevront aucune notification par e-mail, y compris les notifications de réponse, les notifications de mention, les alertes administrateur et les e-mails de synthèse. Un badge rouge « Email Suppressed » apparaîtra à côté des utilisateurs et modérateurs concernés dans l'interface d'administration.

#### Consulter les e-mails supprimés

Les administrateurs de tenant disposant de la permission Manage Data peuvent consulter les e-mails supprimés sur la page [Suppressed Emails](https://fastcomments.com/auth/my-account/suppressed-emails), sous Manage Data.

La page affiche un tableau de toutes les adresses e-mail supprimées associées aux utilisateurs, modérateurs et commentateurs de votre tenant. Vous pouvez filtrer par adresse e-mail à l'aide du champ de recherche.

#### Supprimer une exclusion

Pour supprimer une exclusion, cliquez sur le bouton **Remove** à côté de l'entrée dans le tableau. Vous serez redirigé vers une page de confirmation affichant les détails de l'exclusion. Cliquez sur **Confirm Removal** pour continuer.

Lorsqu'une exclusion est supprimée, FastComments contacte le fournisseur d'e-mails pour débloquer l'adresse et efface le drapeau d'exclusion de tous les enregistrements d'utilisateurs et modérateurs associés.

#### Limites de débit

Pour prévenir les abus, les suppressions sont limitées en fréquence :

- Chaque adresse e-mail ne peut être réactivée qu'une fois tous les 30 jours.
- Chaque tenant peut effectuer jusqu'à 5 suppressions par mois calendaire.

Si une exclusion réapparaît après sa suppression, cela signifie que l'adresse e-mail a de nouveau rebondi ou a été signalée comme spam. Dans ce cas, le problème de délivrabilité sous-jacent doit être résolu avant de tenter une nouvelle suppression.