FastComments authentifie les requêtes vers votre compte afin de vérifier qu’elles proviennent de votre site. C’est pourquoi nous devons connaître le ou les sites sur lesquels vous souhaitez installer FastComments.

FastComments prend en charge l’authentification par domaine, ainsi que par sous‑domaines.

Prenons le site `https://example.com`. Dans ce cas, "`example.com`" est le domaine. `example.com` prend en charge à la fois `example.com` et `www.example.com`. Nous appellerons le « www » le « sous‑domaine ».

Par exemple :

- Pour autoriser uniquement `blog.example.com` :
  - Ajoutez `blog.example.com` à vos domaines.
- Pour autoriser `www.example.com`, `somesite.example.com` et `example.com` :
  - Ajoutez `example.com` à vos domaines.
  - Cela est facturé comme **un domaine** associé à votre compte.
- Vous pouvez désormais ajouter des sous‑domaines génériques, par exemple *myname.vercel.app*.
  - Cela est facturé comme **un domaine** associé à votre compte.

Si vous utilisiez une plateforme de blog et que l’on vous attribuait un sous‑domaine, vous devrez ajouter le **domaine complet incluant le sous‑domaine** à votre compte, par exemple : `cats.blogger.com`.

Nous pouvons ajouter des domaines à notre compte en visitant la page `My Domains` et en cliquant sur `Add a Domain` en bas :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Page My Domains répertoriant les domaines du compte, avec le bouton Ajouter un domaine en bas'; title='La page My Domains' app-screenshot-end]

Pendant la période d’essai, **les domaines sont ajoutés automatiquement à votre compte** lorsque les requêtes proviennent de ces domaines. Cependant, après cette période ils doivent être ajoutés explicitement pour des raisons de sécurité. Vous devriez recevoir un e‑mail lorsque ce comportement automatisé se produit.

Vous **n’avez pas** besoin d’ajouter `localhost` pour le développement local — il est autorisé par défaut.

#### Via The API

Les domaines peuvent également être ajoutés et configurés [via the DomainConfigs API](/guide-api.html#domain-config-structure).