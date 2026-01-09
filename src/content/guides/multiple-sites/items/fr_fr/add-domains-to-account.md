FastComments authentifie les requêtes vers votre compte pour vérifier qu'elles proviennent de votre site. C'est pourquoi
nous devons savoir sur quel site, ou quels sites, vous souhaitez installer FastComments.

FastComments prend en charge l'authentification par domaine ainsi que par sous-domaines.

Prenons le site `https://example.com`. Dans ce cas, "`example.com`" est le domaine. `example.com` prend en charge à la fois `example.com` et `www.example.com`. Nous appellerons le "www" le "sous-domaine".

Par exemple :

- Pour autoriser uniquement `blog.example.com` :
  - Ajoutez `blog.example.com` à vos domaines.
- Pour autoriser `www.example.com`, `somesite.example.com`, et `example.com` :
  - Ajoutez `example.com` à vos domaines.
  - Cela est facturé comme ayant **un domaine** associé à votre compte.
- Vous pouvez désormais ajouter des sous-domaines génériques, par exemple *myname.vercel.app.
  - Cela est facturé comme ayant **un domaine** associé à votre compte.

Si vous utilisez une plateforme de blog et que l'on vous a attribué un sous-domaine, vous devriez ajouter à votre compte le **domaine complet incluant le sous-domaine**, par exemple : `cats.blogger.com`.

Nous pouvons ajouter des domaines à notre compte en visitant la page `My Domains` et en cliquant sur `Add a Domain` en bas :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Pendant la période d'essai, **les domaines sont automatiquement ajoutés à votre compte** lorsque des requêtes proviennent de ces domaines. Cependant, après cette période ils doivent être ajoutés explicitement pour des raisons de sécurité. Vous devriez recevoir un e-mail lorsque ce comportement automatisé se produit.

Vous n'avez **pas** à ajouter `localhost` pour le développement local - il est autorisé par défaut.

#### Via l'API

Les domaines peuvent également être ajoutés et configurés [via l'API DomainConfigs](/guide-api.html#domain-config-structure).

---