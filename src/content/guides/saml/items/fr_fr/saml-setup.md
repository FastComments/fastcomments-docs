La configuration de l’authentification SAML dans FastComments nécessite à la fois une configuration dans votre tableau de bord administrateur et une configuration dans votre fournisseur d’identité.

### Prérequis

Avant de configurer SAML, assurez-vous d’avoir :

- Un forfait FastComments Flex ou Pro (SAML n’est pas disponible sur le forfait Creators)
- Un accès administratif à votre compte FastComments
- Un accès administratif à votre fournisseur d’identité
- Les métadonnées SAML ou les informations de certificat de votre IdP

### Accéder à la configuration SAML

1. Connectez-vous à votre [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Allez dans **API/SSO Settings** dans la barre latérale gauche
3. Cliquez sur le bouton **SAML Config**

Si vous ne voyez pas le bouton SAML Config, vérifiez que :
- Votre compte dispose du forfait requis (Flex ou Pro)
- Vous avez les permissions administratives
- Votre utilisateur a les rôles API Admin ou Admin Admin

### Configuration SAML de base

#### Activer l’authentification SAML

1. Cochez la case **Enable SAML Authentication**
2. Cela active SAML pour votre tenant et rend les champs de configuration disponibles

#### Champs requis

**IdP Single Sign-On URL** *(Requis)*
- L’URL vers laquelle les utilisateurs seront redirigés pour l’authentification
- Généralement fournie par votre fournisseur d’identité
- Exemple : `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Requis)*
- Le certificat public de votre fournisseur d’identité
- Utilisé pour vérifier l’authenticité des réponses SAML
- Doit inclure le certificat complet avec les marqueurs BEGIN/END
- Exemple de format :
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Champs optionnels

**IdP Entity ID / Issuer**
- Identifie votre fournisseur d’identité
- Si laissé vide, la valeur par défaut est l’URL de votre FastComments
- Doit correspondre à l’issuer configuré dans votre IdP

### Configuration avancée

#### Paramètres de sécurité

**Signature Algorithm**
- Par défaut SHA-256 (recommandé)
- Options : SHA-1, SHA-256, SHA-512
- Doit correspondre à la configuration de votre IdP

**Digest Algorithm**
- Par défaut SHA-256 (recommandé)
- Utilisé pour le calcul du digest dans les réponses SAML
- Doit correspondre à la configuration de votre IdP

**Name ID Format**
- Par défaut au format Email Address
- Détermine comment les identifiants utilisateur sont formatés
- Options courantes : Email Address, Persistent, Transient

#### Chiffrement (Optionnel)

**Private Key for Decryption**
- Nécessaire uniquement si votre IdP chiffre les assertions SAML
- Collez votre clé privée utilisée pour le déchiffrement
- La plupart des déploiements n’exigent pas le chiffrement des assertions

### Enregistrement de la configuration

1. Vérifiez l’exactitude de tous les paramètres
2. Cliquez sur **Save SAML Configuration**
3. Le système validera votre configuration
4. Si la validation réussit, vous verrez un message de confirmation

### Étapes suivantes

Après avoir enregistré votre configuration SAML FastComments :

1. Configurez votre fournisseur d’identité en utilisant les informations du Fournisseur de service
2. Testez le flux d’authentification
3. Configurez les rôles et permissions des utilisateurs selon vos besoins

Les informations du Fournisseur de service nécessaires à la configuration de votre IdP seront affichées une fois que SAML sera activé.