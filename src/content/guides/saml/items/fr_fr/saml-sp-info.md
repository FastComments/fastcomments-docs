Lorsque SAML est activé dans FastComments, le système génère automatiquement les informations du fournisseur de services (SP) que vous devez configurer dans votre fournisseur d'identité.

### Accéder aux informations du fournisseur de services

Les informations SP sont affichées sur votre page de configuration SAML après l'activation de l'authentification SAML. Ces informations incluent tous les détails dont votre fournisseur d'identité a besoin pour établir la relation de confiance SAML.

### Points de terminaison du fournisseur de services

#### ID d'entité SP / Audience
**But** : Identifie de manière unique votre instance FastComments en tant que fournisseur de services  
**Format** : `https://fastcomments.com/saml/{your-tenant-id}`  
**Utilisation** : Configurez ceci comme l'Entity ID ou Audience dans votre IdP

Cet identifiant garantit que les réponses SAML sont destinées à votre locataire FastComments spécifique et empêche que des réponses SAML soient acceptées par d'autres instances.

#### URL Assertion Consumer Service (ACS)
**But** : Le point de terminaison où votre IdP envoie les réponses SAML après l'authentification de l'utilisateur  
**Format** : `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Utilisation** : Configurez ceci comme l'ACS URL ou Reply URL dans votre IdP

C'est l'endroit où les utilisateurs sont redirigés après une authentification réussie avec votre fournisseur d'identité, avec l'assertion SAML contenant les informations utilisateur.

#### URL des métadonnées SP
**But** : Fournit la configuration SAML complète au format XML standard  
**Format** : `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Utilisation** : Certains IdP peuvent importer automatiquement la configuration en utilisant cette URL

L'URL des métadonnées contient toutes les informations nécessaires du SP au format XML, ce qui facilite la configuration automatique des fournisseurs d'identité compatibles.

#### URL de connexion SAML
**But** : Lien direct pour initier l'authentification SAML pour votre locataire  
**Format** : `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Utilisation** : Lien direct pour rediriger les utilisateurs vers l'authentification SAML ou tester le flux

Vous pouvez utiliser cette URL pour tester l'authentification SAML ou fournir aux utilisateurs un lien direct pour se connecter via SAML.

### Prise en charge des liaisons SAML

FastComments prend en charge les liaisons SAML suivantes :

#### Liaison HTTP-POST
- **Méthode principale** : Liaison la plus courante pour les réponses SAML  
- **Sécurité** : La réponse SAML est envoyée via HTTP POST vers l'ACS URL  
- **Utilisation** : Recommandée pour les déploiements en production

#### Liaison HTTP-Redirect
- **Méthode alternative** : Réponse SAML envoyée via redirection HTTP  
- **Limitations** : Taille de charge utile limitée en raison des restrictions de longueur d'URL  
- **Utilisation** : Prise en charge, mais HTTP-POST est préféré

### Politique de Name ID

FastComments configure la politique de Name ID suivante dans les requêtes SAML :

- **Format par défaut** : `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Formats alternatifs** : Persistent, Transient, Unspecified (configurable)  
- **Exigence** : L'adresse e-mail est utilisée comme identifiant principal de l'utilisateur

### Attributs de requête SAML

Lors de l'initiation de l'authentification SAML, FastComments envoie des requêtes avec ces caractéristiques :

#### Signature des requêtes
- **Statut** : Optionnelle (configurable)  
- **Algorithme** : Correspond à l'algorithme de signature configuré  
- **Certificat** : Utilise le certificat spécifique au locataire si la signature des requêtes est activée

#### Attributs demandés
FastComments demande les attributs suivants dans les AuthnRequests SAML :

- **Email** : Requis pour l'identification de l'utilisateur  
- **Prénom** : Optionnel pour l'affichage  
- **Nom** : Optionnel pour l'affichage  
- **Rôles/Groupes** : Optionnels pour le contrôle d'accès et les permissions

### Copier les informations SP

La page de configuration SAML fournit des champs cliquables qui copient automatiquement les informations SP dans votre presse-papiers :

1. Cliquez sur n'importe quel champ d'information SP (Entity ID, ACS URL, etc.)  
2. La valeur est automatiquement copiée dans votre presse-papiers  
3. Collez la valeur dans la configuration de votre fournisseur d'identité  
4. Une brève mise en surbrillance indique la copie réussie

Cela facilite le transfert précis des informations SP vers votre IdP sans erreurs de saisie.

### Informations sur le certificat SP

#### Utilisation du certificat
- **But** : Chiffre les communications et vérifie l'identité du SP  
- **Rotation** : Les certificats sont gérés automatiquement par FastComments  
- **Accès** : Les certificats publics sont disponibles via l'URL des métadonnées

#### Détails du certificat
- **Algorithme** : RSA-2048 ou supérieur  
- **Validité** : Les certificats sont automatiquement renouvelés avant leur expiration  
- **Distribution** : Disponible via les métadonnées SAML standard

### Dépannage de la configuration SP

Si votre fournisseur d'identité signale des problèmes avec les informations SP :

1. **Vérifiez les URL** : Assurez-vous que toutes les URL utilisent HTTPS et incluent le bon tenant ID  
2. **Vérifiez les métadonnées** : Utilisez l'URL des métadonnées pour vérifier la configuration  
3. **Testez la connectivité** : Assurez-vous que votre IdP peut atteindre les points de terminaison FastComments  
4. **Validez le format** : Confirmez que votre IdP prend en charge le format des informations SP

Les problèmes courants incluent :  
- ID de locataire incorrect dans les URL  
- Problèmes de connectivité réseau entre l'IdP et FastComments  
- L'IdP attend un format d'URL différent ou des options de configuration supplémentaires