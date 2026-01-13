Lorsque SAML est activé dans FastComments, le système génère automatiquement les informations du Fournisseur de services (SP) que vous devez configurer dans votre fournisseur d'identité.

### Accéder aux informations du Fournisseur de services

Les informations du SP sont affichées sur votre page de configuration SAML après l'activation de l'authentification SAML. Ces informations incluent tous les détails dont votre fournisseur d'identité a besoin pour établir la relation de confiance SAML.

### Points de terminaison du Fournisseur de services

#### SP Entity ID / Audience
**Objectif**: Identifie de façon unique votre instance FastComments en tant que fournisseur de services  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Utilisation**: Configurez ceci comme Entity ID ou Audience dans votre IdP

Cet identifiant garantit que les réponses SAML sont destinées à votre locataire FastComments spécifique et empêche que des réponses SAML soient acceptées par d'autres instances.

#### Assertion Consumer Service (ACS) URL
**Objectif**: Le point de terminaison où votre IdP envoie les réponses SAML après l'authentification de l'utilisateur  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Utilisation**: Configurez ceci comme ACS URL ou Reply URL dans votre IdP

C'est l'endroit où les utilisateurs sont redirigés après une authentification réussie auprès de votre fournisseur d'identité, accompagné de l'assertion SAML contenant les informations utilisateur.

#### SP Metadata URL
**Objectif**: Fournit la configuration SAML complète au format XML standard  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Utilisation**: Certains IdP peuvent importer automatiquement la configuration en utilisant cette URL

L'URL des métadonnées contient toutes les informations nécessaires du SP au format XML, ce qui facilite la configuration automatique des fournisseurs d'identité compatibles.

#### SAML Login URL
**Objectif**: Lien direct pour initier l'authentification SAML pour votre locataire  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Utilisation**: Liez les utilisateurs directement à l'authentification SAML ou testez le flux

Vous pouvez utiliser cette URL pour tester l'authentification SAML ou fournir aux utilisateurs un lien direct pour se connecter via SAML.

### Prise en charge des bindings SAML

FastComments prend en charge les bindings SAML suivants :

#### HTTP-POST Binding
- **Méthode principale**: Binding le plus courant pour les réponses SAML  
- **Sécurité**: La réponse SAML est envoyée via HTTP POST à l'ACS URL  
- **Utilisation**: Recommandé pour les déploiements en production

#### HTTP-Redirect Binding
- **Méthode alternative**: Réponse SAML envoyée via redirection HTTP  
- **Limitations**: Taille de charge utile limitée à cause des restrictions de longueur d'URL  
- **Utilisation**: Pris en charge mais HTTP-POST est préféré

### Politique Name ID

FastComments configure la politique Name ID suivante dans les requêtes SAML :

- **Format par défaut**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Formats alternatifs**: Persistent, Transient, Unspecified (configurable)  
- **Exigence**: L'adresse courriel est utilisée comme identifiant principal de l'utilisateur

### Attributs de la requête SAML

Lors de l'initiation de l'authentification SAML, FastComments envoie des requêtes avec ces caractéristiques :

#### Signature de la requête
- **Statut**: Optionnel (configurable)  
- **Algorithme**: Correspond à l'algorithme de signature configuré  
- **Certificat**: Utilise le certificat spécifique au locataire si la signature des requêtes est activée

#### Attributs demandés
FastComments demande les attributs suivants dans les SAML AuthnRequests :

- **Email**: Requis pour l'identification de l'utilisateur  
- **Prénom**: Optionnel pour l'affichage  
- **Nom de famille**: Optionnel pour l'affichage  
- **Rôles/Groupes**: Optionnel pour le contrôle d'accès et les permissions

### Copier les informations du SP

La page de configuration SAML fournit des champs cliquables qui copient automatiquement les informations du SP dans votre presse-papiers :

1. Cliquez sur n'importe quel champ d'information du SP (Entity ID, ACS URL, etc.)  
2. La valeur est automatiquement copiée dans votre presse-papiers  
3. Collez la valeur dans la configuration de votre fournisseur d'identité  
4. Un bref surlignage indique la copie réussie

Cela facilite le transfert précis des informations du SP vers votre IdP sans erreurs de saisie.

### Informations sur le certificat du SP

#### Utilisation du certificat
- **Objectif**: Chiffre les communications et vérifie l'identité du SP  
- **Rotation**: Les certificats sont gérés automatiquement par FastComments  
- **Accès**: Les certificats publics sont disponibles via l'URL des métadonnées

#### Détails du certificat
- **Algorithme**: RSA-2048 ou supérieur  
- **Validité**: Les certificats sont renouvelés automatiquement avant expiration  
- **Distribution**: Disponible via les métadonnées SAML standard

### Dépannage de la configuration du SP

Si votre fournisseur d'identité signale des problèmes avec les informations du SP :

1. **Vérifiez les URL**: Assurez-vous que toutes les URL utilisent HTTPS et incluent le bon ID du locataire  
2. **Vérifiez les métadonnées**: Utilisez l'URL des métadonnées pour vérifier la configuration  
3. **Testez la connectivité**: Assurez-vous que votre IdP peut atteindre les points de terminaison FastComments  
4. **Validez le format**: Confirmez que votre IdP prend en charge le format des informations du SP

Les problèmes courants incluent :
- ID du locataire incorrect dans les URL  
- Problèmes de connectivité réseau entre l'IdP et FastComments  
- IdP attendant des formats d'URL différents ou des options de configuration supplémentaires