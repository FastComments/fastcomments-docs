FastComments propose à la fois l'authentification SSO et SAML. Comprendre les différences vous aide à choisir l'approche adaptée à votre organisation.

### Productions SSO simples/sécurisées

FastComments propose deux flux SSO différents pour s'authentifier dans le widget de commentaires depuis votre site.
Ceci est différent de SAML, et ne nécessite pas SAML. Au lieu de cela, Simple SSO exige simplement de passer un objet au widget de commentaires, tandis que Secure SSO fait cela plus le hachage de la charge utile avec une clé API.

SAML, en revanche, authentifie l'utilisateur pour l'ensemble du produit (en fonction de ses permissions) *ainsi que* le widget de commentaires (si l'utilisateur a activé les cookies tiers pour notre domaine).

### Authentification SAML

SAML est un protocole d'authentification de niveau entreprise qui offre des capacités d'intégration et de sécurité plus robustes :

- **Implémentation** : Nécessite la configuration du fournisseur d'identité (IdP) et l'échange de certificats
- **Sécurité** : Utilise des assertions XML signées et prend en charge le chiffrement
- **Cas d'utilisation** : Idéal pour les entreprises disposant d'une infrastructure SAML existante (Active Directory, Okta, etc.)
- **Complexité de mise en place** : Plus impliqué - nécessite la configuration de l'IdP et la gestion des certificats
- **Fonctionnalités d'entreprise** : Mappage avancé des rôles, gestion centralisée des utilisateurs, pistes d'audit

### Quand choisir SAML

Envisagez l'authentification SAML si votre organisation :

- Utilise déjà un fournisseur d'identité compatible SAML (Okta, Azure AD, ADFS, etc.)
- Exige une sécurité et une conformité de niveau entreprise
- A besoin d'une gestion centralisée des utilisateurs et du contrôle d'accès
- A plusieurs applications utilisant SAML pour l'authentification
- Nécessite des pistes d'audit détaillées et des rapports de sécurité

### Quand choisir Simple ou Secure SSO

Nos solutions SSO axées sur le widget peuvent être suffisantes si vous :

- Disposez d'un système d'authentification personnalisé
- Avez besoin d'une mise en œuvre rapide avec une configuration minimale
- N'avez pas besoin d'intégration à un fournisseur d'identité d'entreprise
- Souhaitez contrôler les données utilisateur directement depuis votre application
- Avez des exigences de sécurité plus simples

Simple et Secure SSO sont couramment utilisés pour les portails en ligne, blogs, etc., où l'utilisateur possède déjà un compte *via votre site ou application* mais n'utilise pas nécessairement SAML.