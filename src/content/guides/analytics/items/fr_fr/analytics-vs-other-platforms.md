Vous pourriez constater que nos métriques d'analytique affichent des chiffres légèrement différents de ceux de, par exemple, Google Ads © ou des produits similaires.

Pour les sites avec un widget de commentaires par page, les chiffres fournis par FastComments Analytics sont très précis, et s'ils sont incorrects, ils seront **inférieurs** à la valeur réelle, mais pas plus.

Si vous avez une SPA, vous pourriez constater que les chiffres de FastComments Analytics sont plus élevés que ceux rapportés par vos produits marketing. C'est parce que le produit marketing peut uniquement suivre quand la page n'est pas chargée, mais pas chaque fois qu'un utilisateur fait quelque chose dans la page qui pourrait déclencher l'affichage d'un nouveau fil de commentaires, ce qui compterait comme un chargement de page pour FastComments Analytics.

#### Informations techniques

FastComments Analytics suit chaque chargement de page et ne s'appuie pas sur le hasard comme optimisation. Chaque chargement de page entraîne la mise à jour d'un compteur en mémoire dans chaque fil sur chaque serveur, qui est ensuite périodiquement persisté dans la base de données via une opération atomique.
