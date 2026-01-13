### Utilisation des API authentifiées (DefaultApi)

**Important :** Vous devez définir votre clé API sur l'ApiClient avant d'effectuer des requêtes authentifiées. Si vous ne le faites pas, les requêtes échoueront avec une erreur 401.

```ruby
require 'fastcomments-client'

# Créez et configurez le client API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBLIGATOIRE : Définissez votre clé API (obtenez-la depuis votre tableau de bord FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Créez l'instance de l'API avec le client configuré
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
  # Erreurs courantes :
  # - 401 : la clé API est manquante ou invalide
  # - 400 : la validation de la requête a échoué
end
```

### Utilisation des API publiques (PublicApi)

Les points de terminaison publics ne requièrent pas d'authentification :

```ruby
require 'fastcomments-client'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problèmes courants

1. **Erreur 401 «missing-api-key»** : Assurez-vous de définir `config.api_key['x-api-key'] = 'YOUR_KEY'` avant de créer l'instance DefaultApi.
2. **Mauvaise classe API** : Utilisez `DefaultApi` pour les requêtes authentifiées côté serveur, `PublicApi` pour les requêtes côté client/public.
3. **Clé API nulle** : Le SDK ignorera silencieusement l'authentification si la clé API est nulle, ce qui entraînera des erreurs 401.