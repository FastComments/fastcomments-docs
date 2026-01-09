FastComments authentifie les requêtes à votre compte pour vérifier qu'elles proviennent de votre site. C'est pourquoi nous devons savoir sur quel site, ou quels sites, vous souhaitez installer FastComments.

FastComments prend en charge l'authentification par domaine ainsi que par sous-domaines.

Let's take the site `https://example.com`. In this case, "`example.com`" is the domain. `example.com` supports both `example.com`, and `www.example.com`. We'll call the "www" the "subdomain".

For Example:

- Pour autoriser uniquement `blog.example.com` :
  - Ajoutez `blog.example.com` à vos domaines.
- Pour permettre `www.example.com`, `somesite.example.com`, et `example.com` :
  - Ajoutez `example.com` à vos domaines.
  - Ceci est facturé comme ayant **un domaine** associé à votre compte.
- Vous pouvez maintenant ajouter des sous-domaines génériques, par exemple *myname.vercel.app. 
  - Ceci est facturé comme ayant **un domaine** associé à votre compte.

Si vous utilisiez une plateforme de blog et qu'on vous a attribué un sous-domaine, vous voudrez ajouter le **domaine complet incluant le sous-domaine** à votre compte, par exemple : `cats.blogger.com`.

Nous pouvons ajouter des domaines à notre compte en visitant la page `My Domains` et en cliquant `Add a Domain` en bas :

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Pendant la période d'essai, **les domaines sont automatiquement ajoutés à votre compte** lorsque des requêtes proviennent de ces domaines. Toutefois, après cette période, ils doivent être ajoutés explicitement pour des raisons de sécurité. Vous devriez recevoir un courriel lorsque ce comportement automatisé se produit.

Vous n'avez **pas** à ajouter `localhost` pour le développement local - il est autorisé par défaut.

#### Via l'API

Les domaines peuvent également être ajoutés et configurés [via the DomainConfigs API](/guide-api.html#domain-config-structure).