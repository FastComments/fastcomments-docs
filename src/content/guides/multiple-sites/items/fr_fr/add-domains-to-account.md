FastComments authentifie les requêtes envoyées à votre compte pour vérifier qu'elles proviennent bien de votre site. C'est pourquoi
nous avons besoin de savoir quel(s) site(s) vous souhaitez installer FastComments.

FastComments prend en charge l'authentification par domaine, ainsi que par sous-domaines.

Prenons le site `https://example.com`. Dans ce cas, « `example.com` » est le domaine. `example.com` prend en charge à la fois `example.com` et `www.example.com`. Nous appellerons le « www » le « sous-domaine ».

Par exemple :

- Pour autoriser uniquement `blog.example.com` :
  - Ajoutez `blog.example.com` à vos domaines.
- Pour autoriser `www.example.com`, `somesite.example.com` et `example.com` :
  - Ajoutez `example.com` à vos domaines.
  - Ceci est facturé comme ayant **un seul domaine** associé à votre compte.
- Vous pouvez désormais ajouter des sous-domaines génériques, par exemple *myname.vercel.app.
  - Ceci est facturé comme ayant **un seul domaine** associé à votre compte.

Si vous utilisiez une plateforme de blog et qu'un sous-domaine vous a été attribué, vous devriez
ajouter le **domaine complet incluant le sous-domaine** à votre compte, par exemple : `cats.blogger.com`.

Nous pouvons ajouter des domaines à notre compte en visitant la page `My Domains` et en cliquant sur `Add a Domain` en bas de page :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Page My Domains listant les domaines du compte, avec le bouton Add a Domain en bas'; title='La page My Domains' app-screenshot-end]

Pendant la période d'essai, **les domaines sont automatiquement ajoutés à votre compte** lorsque des requêtes proviennent de ces domaines. Cependant,
après cette période, ils doivent être ajoutés explicitement pour des raisons de sécurité. Vous devriez recevoir un e-mail lorsque ce comportement automatisé se produit.

Vous n'avez **pas** besoin d'ajouter `localhost` pour le développement local - il est autorisé par défaut.

#### Via l'API

Les domaines peuvent également être ajoutés et configurés [via l'API DomainConfigs](/guide-api.html#domain-config-structure).