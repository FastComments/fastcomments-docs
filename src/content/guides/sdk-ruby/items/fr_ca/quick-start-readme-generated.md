### Utilisation des API authentifiées (DefaultApi)

**Important :** Vous devez définir votre clé d'API sur l'ApiClient avant d'effectuer des requêtes authentifiées. Sinon, les requêtes échoueront avec une erreur 401.

```ruby
require 'fastcomments'

# Créez et configurez le client API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIS : Définissez votre clé d'API (obtenez-la depuis votre tableau de bord FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Créez l'instance API avec le client configuré
api = FastCommentsClient::DefaultApi.new(api_client)

# Vous pouvez maintenant effectuer des appels API authentifiés
begin
  # Exemple : Ajouter un utilisateur SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Erreurs communes :
  # - 401 : la clé d'API est manquante ou invalide
  # - 400 : la validation de la requête a échoué
end
```

### Utilisation des API publiques (PublicApi)

Les points de terminaison publics ne nécessitent pas d'authentification :

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Utilisation des API de modération (ModerationApi)

Les méthodes de modération alimentent le tableau de bord du modérateur. Transmettez un jeton `sso` afin que la requête soit effectuée au nom d'un modérateur authentifié via SSO :

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Exemple : Lister les commentaires dans la file d’attente de modération
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problèmes courants

1. **401 "missing-api-key" error** : Assurez-vous de définir `config.api_key['x-api-key'] = 'YOUR_KEY'` avant de créer l'instance DefaultApi.
2. **Wrong API class** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public, et `ModerationApi` pour les requêtes du tableau de bord du modérateur.
3. **Null API key** : Le SDK ignorera silencieusement l'authentification si la clé d'API est nulle, entraînant des erreurs 401.