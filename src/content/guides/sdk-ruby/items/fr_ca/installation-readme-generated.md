Ajoutez cette ligne au Gemfile de votre application :

```ruby
gem 'fastcomments'
```

Et ensuite exécutez :

```bash
bundle install
```

Ou installez-le vous-même comme suit :

```bash
gem install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client d'API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [Documentation de la bibliothèque cliente de l'API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe deux classes, `DefaultApi` et `PublicApi`. La `DefaultApi` contient des méthodes qui requièrent votre clé API, et `PublicApi` contient des appels d'API
qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc., sans authentification.