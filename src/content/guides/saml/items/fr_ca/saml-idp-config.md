Après avoir configuré SAML dans FastComments, vous devez configurer FastComments en tant que fournisseur de services (Service Provider) dans votre fournisseur d'identité.

### Configuration générale de l'IdP

La plupart des fournisseurs d'identité exigent les informations suivantes pour ajouter FastComments en tant qu'application SAML :

#### Informations requises du fournisseur de services

Ces valeurs sont générées automatiquement et affichées dans votre page de configuration SAML de FastComments :

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Cela identifie de manière unique votre instance FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Là où votre IdP envoie les réponses SAML après authentification

**SP Metadata URL** *(si pris en charge par votre IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Fournit la configuration SAML complète au format XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Lien direct pour initier l'authentification SAML

### Attributs SAML requis

Configurez votre fournisseur d'identité pour envoyer ces attributs avec les réponses SAML :

#### Attributs essentiels

**Adresse courriel** *(Requise)*
- **Nom d'attribut**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Objectif**: identification unique de l'utilisateur et notifications
- **Format**: adresse courriel valide

#### Attributs optionnels

**Prénom**
- **Noms d'attribut**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Objectif**: nom d'affichage de l'utilisateur

**Nom de famille**
- **Noms d'attribut**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Objectif**: nom d'affichage de l'utilisateur

**Rôles** *(Important pour le contrôle d'accès)*
- **Noms d'attribut**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Objectif**: attribution des rôles et permissions dans FastComments
- **Format**: tableau de chaînes de rôles ou valeurs séparées par des virgules

### Configurations courantes des fournisseurs d'identité

#### Microsoft Azure AD

1. **Ajouter une application d'entreprise**
   - Recherchez "FastComments" ou créez une application SAML personnalisée
   - Utilisez les informations SP fournies par FastComments

2. **Configurer les attributs**
   - Courriel : `user.mail` ou `user.userprincipalname`
   - Prénom : `user.givenname`
   - Nom de famille : `user.surname`
   - Rôles : `user.assignedroles` ou groupes d'annuaire

#### Okta

1. **Créer une application SAML**
   - Utilisez "Create New App" et sélectionnez SAML 2.0
   - Configurez avec les informations SP de FastComments

2. **Déclarations d'attributs**
   - Courriel : `user.email`
   - Prénom : `user.firstName`
   - Nom de famille : `user.lastName`
   - Rôles : `user.groups` ou attributs personnalisés

#### Google Workspace

1. **Ajouter une application SAML**
   - Allez dans Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configurez avec les informations SP de FastComments

2. **Mappage des attributs**
   - Courriel : Primary email
   - Prénom : First name
   - Nom de famille : Last name
   - Rôles : Groups ou attributs personnalisés

#### Active Directory Federation Services (ADFS)

1. **Ajouter un Relying Party Trust**
   - Utilisez l'URL de métadonnées FastComments ou une configuration manuelle
   - Configurez les informations SP comme fourni

2. **Règles de revendication**
   - Courriel : revendication "Email Address"
   - Nom : revendication "Name ID"
   - Rôles : appartenance à un groupe ou revendications personnalisées

### Flexibilité des noms d'attributs

FastComments accepte les informations de rôle provenant de plusieurs noms d'attributs pour s'adapter aux différentes configurations d'IdP :

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Cette flexibilité garantit la compatibilité avec divers fournisseurs d'identité sans exiger des conventions de nommage d'attributs spécifiques.

### Tester votre configuration

Après avoir configuré votre fournisseur d'identité :

1. Enregistrez la configuration de l'IdP
2. Testez avec un compte utilisateur de test dédié
3. Vérifiez que les attributs sont envoyés correctement
4. Vérifiez que les rôles sont correctement mappés
5. Assurez-vous que le flux d'authentification se termine avec succès

La plupart des fournisseurs d'identité proposent des outils de test SAML pour valider la configuration avant de la déployer aux utilisateurs en production.