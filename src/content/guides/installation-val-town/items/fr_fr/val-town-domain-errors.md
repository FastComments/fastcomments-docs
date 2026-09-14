Une fois que vous désactivez le locataire `demo`, le widget peut refuser de se charger avec une erreur d'autorisation. Cela est dû au fait que FastComments ne sait pas qu'il doit autoriser votre compte à être utilisé sur ce domaine.

[Allez ici pour ajouter votre site à votre compte.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town mérite un second examen ici, car un val peut être accessible à partir de plusieurs noms d'hôte :

- Chaque val HTTP possède un point de terminaison par défaut long, `<org>--<id>.web.val.run`.
- Revendiquer un sous-domaine personnalisé ajoute `<name>.val.run`.
- Un [domaine personnalisé](https://docs.val.town/vals/http/custom-domains/) ajoute un troisième.
- Les branches obtiennent leurs propres URL.

Ajoutez les noms d'hôte depuis lesquels vous servez réellement le widget. Si vous revendiquez un sous-domaine après avoir configuré les choses, ajoutez-le également, sinon le widget fonctionnera sur l'ancienne URL et échouera sur la nouvelle.