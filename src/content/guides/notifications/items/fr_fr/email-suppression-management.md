---
Lorsque des e-mails envoyés par FastComments rebondissent ou sont signalés comme spam par le destinataire, le fournisseur d'e-mail ajoute cette adresse à une liste de suppression. FastComments synchronise quotidiennement ces listes de suppression afin que d'autres e-mails ne soient pas envoyés à des adresses qui ne peuvent pas les recevoir.

Les utilisateurs et les modérateurs dont l'adresse e-mail est supprimée ne recevront aucune notification par e-mail, y compris les notifications de réponse, les notifications de mention, les alertes administratives et les e-mails récapitulatifs. Un badge rouge "E-mail supprimé" apparaîtra à côté des utilisateurs et des modérateurs concernés dans l'interface d'administration.

#### Viewing Suppressed Emails

Les administrateurs du locataire disposant de l'autorisation Gérer les données peuvent consulter les e-mails supprimés sur la page [E-mails supprimés](https://fastcomments.com/auth/my-account/suppressed-emails), sous Gérer les données.

La page affiche un tableau de toutes les adresses e-mail supprimées associées aux utilisateurs, aux modérateurs et aux commentateurs de votre locataire. Vous pouvez filtrer par adresse e-mail à l'aide du champ de recherche.

#### Retirer une suppression

Pour retirer une suppression, cliquez sur le bouton **Supprimer** à côté de l'entrée dans le tableau. Vous serez redirigé vers une page de confirmation affichant les détails de la suppression. Cliquez sur **Confirmer la suppression** pour continuer.

Lorsqu'une suppression est retirée, FastComments contacte le fournisseur d'e-mail pour débloquer l'adresse et supprime le drapeau de suppression de tous les enregistrements d'utilisateurs et de modérateurs associés.

#### Limites de débit

Pour prévenir les abus, les retraits sont soumis à des limites de débit :

- Chaque adresse e-mail ne peut être retirée de la liste de suppression qu'une fois tous les 30 jours.
- Chaque locataire peut effectuer jusqu'à 5 retraits par mois civil.

Si une suppression réapparaît après son retrait, cela signifie que l'adresse e-mail a de nouveau rebondi ou a été signalée comme spam. Dans ce cas, le problème de délivrabilité sous-jacent doit être résolu avant de tenter un nouveau retrait.

---