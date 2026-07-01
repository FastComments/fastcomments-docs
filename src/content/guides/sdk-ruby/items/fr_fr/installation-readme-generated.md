Ajoutez cette ligne au Gemfile de votre application :

```ruby
gem 'fastcomments'
```

Ensuite, exécutez :

```bash
bundle install
```

Ou installez‑le vous-même avec :

```bash
gem install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l’utilisation de l’API.

- [Documentation du client API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels API pouvant être effectués directement depuis un navigateur, un appareil mobile, etc., sans authentification. `ModerationApi` regroupe les méthodes qui alimentent le tableau de bord des modérateurs.

`ModerationApi` propose une suite étendue d’API de modération en temps réel et rapides. Chaque méthode de `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.