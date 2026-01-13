Après avoir configuré SAML dans FastComments, vous devez configurer FastComments en tant que fournisseur de services (Service Provider) dans votre fournisseur d'identité.

### Configuration générale du fournisseur d'identité

La plupart des fournisseurs d'identité exigent les informations suivantes pour ajouter FastComments en tant qu'application SAML :

#### Informations requises sur le fournisseur de services

Ces valeurs sont générées automatiquement et affichées dans votre page de configuration SAML FastComments :

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Identifie de manière unique votre instance FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Où votre IdP envoie les réponses SAML après authentification

**SP Metadata URL** *(si votre IdP le prend en charge)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Fournit la configuration SAML complète au format XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Lien direct pour initier l'authentification SAML

### Attributs SAML requis

Configurez votre fournisseur d'identité pour envoyer ces attributs avec les réponses SAML :

#### Attributs essentiels

**Adresse e-mail** *(Obligatoire)*
- **Nom d'attribut**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **But**: Identification unique de l'utilisateur et notifications
- **Format**: Adresse e-mail valide

#### Attributs optionnels

**Prénom**
- **Noms d'attributs**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **But**: Nom affiché de l'utilisateur

**Nom**
- **Noms d'attributs**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **But**: Nom affiché de l'utilisateur

**Rôles** *(Important pour le contrôle d'accès)*
- **Noms d'attributs**: `roles`, `groups`, `memberOf`, or custom attribute names
- **But**: Attribution des rôles et permissions dans FastComments
- **Format**: Tableau de chaînes de rôles ou valeurs séparées par des virgules

### Configurations courantes des fournisseurs d'identité

#### Microsoft Azure AD

1. **Ajouter une application d'entreprise**
   - Recherchez "FastComments" ou créez une application SAML personnalisée
   - Utilisez les informations SP fournies par FastComments

2. **Configurer les attributs**
   - Email: `user.mail` or `user.userprincipalname`
   - Prénom: `user.givenname`
   - Nom: `user.surname`
   - Rôles: `user.assignedroles` or directory groups

#### Okta

1. **Créer une application SAML**
   - Utilisez "Create New App" et sélectionnez SAML 2.0
   - Configurez avec les informations SP de FastComments

2. **Déclarations d'attributs**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Rôles: `user.groups` or custom attributes

#### Google Workspace

1. **Ajouter une application SAML**
   - Allez dans Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configurez avec les informations SP de FastComments

2. **Mappage des attributs**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Rôles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Ajouter une Relying Party Trust**
   - Utilisez l'URL des métadonnées FastComments ou une configuration manuelle
   - Configurez les informations SP fournies

2. **Règles d'émission de revendications**
   - Email: Email Address claim
   - Name: Name ID claim
   - Rôles: Group membership or custom claims

### Flexibilité des noms d'attribut

FastComments accepte les informations de rôle provenant de plusieurs noms d'attribut pour s'adapter aux différentes configurations d'IdP :

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Cette flexibilité garantit la compatibilité avec divers fournisseurs d'identité sans exiger de conventions de nommage d'attributs spécifiques.

### Tester votre configuration

Après avoir configuré votre fournisseur d'identité :

1. Enregistrez la configuration de l'IdP
2. Testez avec un compte utilisateur de test dédié
3. Vérifiez que les attributs sont envoyés correctement
4. Vérifiez que les rôles sont correctement mappés
5. Assurez-vous que le flux d'authentification se termine avec succès

La plupart des fournisseurs d'identité proposent des outils de test SAML pour valider la configuration avant le déploiement auprès des utilisateurs en production.