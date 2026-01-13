La configuration de l’authentification SAML dans FastComments nécessite à la fois une configuration dans votre tableau de bord administrateur et une configuration dans votre fournisseur d’identité.

### Prérequis

Avant de configurer SAML, assurez-vous d’avoir :

- Un plan FastComments Flex ou Pro (SAML n’est pas disponible sur le plan Creators)
- Un accès administratif à votre compte FastComments
- Un accès administratif à votre fournisseur d’identité
- Les métadonnées SAML ou les informations de certificat de votre IdP

### Accéder à la configuration SAML

1. Connectez-vous à votre [tableau de bord administrateur FastComments](https://fastcomments.com/auth/my-account)
2. Allez dans **Paramètres API/SSO** dans la barre latérale gauche
3. Cliquez sur le bouton **Configuration SAML**

Si vous ne voyez pas le bouton Configuration SAML, vérifiez que :
- Votre compte dispose du forfait requis (Flex ou Pro)
- Vous avez des permissions administratives
- Votre utilisateur a les rôles API Admin ou Admin Admin

### Configuration SAML de base

#### Activer l’authentification SAML

1. Cochez la case **Activer l’authentification SAML**
2. Cela active SAML pour votre locataire et rend les champs de configuration disponibles

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
- Si laissé vide, la valeur par défaut est votre URL FastComments
- Devrait correspondre à l’issuer configuré dans votre IdP

### Configuration avancée

#### Paramètres de sécurité

**Signature Algorithm**
- Par défaut SHA-256 (recommandé)
- Options : SHA-1, SHA-256, SHA-512
- Devrait correspondre à la configuration de votre IdP

**Digest Algorithm**
- Par défaut SHA-256 (recommandé)
- Utilisé pour le calcul du digest dans les réponses SAML
- Devrait correspondre à la configuration de votre IdP

**Name ID Format**
- Par défaut le format Adresse e-mail
- Détermine comment les identifiants utilisateur sont formatés
- Options courantes : Adresse e-mail, Persistent, Transient

#### Chiffrement (optionnel)

**Private Key for Decryption**
- Nécessaire uniquement si votre IdP chiffre les assertions SAML
- Collez votre clé privée utilisée pour le déchiffrement
- La plupart des déploiements n’exigent pas le chiffrement des assertions

### Enregistrement de la configuration

1. Vérifiez l’exactitude de tous les paramètres
2. Cliquez sur **Enregistrer la configuration SAML**
3. Le système validera votre configuration
4. Si la validation réussit, vous verrez un message de confirmation

### Étapes suivantes

Après avoir enregistré votre configuration SAML FastComments :

1. Configurez votre fournisseur d’identité en utilisant les informations du Service Provider
2. Testez le flux d’authentification
3. Configurez les rôles et permissions des utilisateurs selon vos besoins

Les informations du Service Provider nécessaires à la configuration de votre IdP s’afficheront une fois que SAML sera activé.