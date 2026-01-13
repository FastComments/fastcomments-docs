FastComments offre à la fois l’authentification SSO et SAML. Comprendre les différences vous aide à choisir l’approche adaptée pour votre organisation.

### SSO simple/sécurisé — Présentation

FastComments propose deux flux SSO différents pour l’authentification dans le widget de commentaires via votre site.
Ceci est différent du SAML et ne nécessite pas SAML. Au lieu de cela, le SSO simple consiste simplement à transmettre un objet au
widget de commentaires, tandis que le SSO sécurisé fait cela en plus de hacher la charge utile avec une clé API.

Le SAML, en revanche, authentifie l’utilisateur pour l’ensemble du produit (en fonction de ses permissions) *ainsi que* le widget de commentaires
(s’ils ont activé les cookies tiers pour notre domaine).

### Authentification SAML

Le SAML est un protocole d’authentification de niveau entreprise qui offre des capacités d’intégration et de sécurité plus robustes :

- **Implémentation** : Nécessite la configuration d’un fournisseur d’identité (IdP) et l’échange de certificats
- **Sécurité** : Utilise des assertions XML signées et prend en charge le chiffrement
- **Cas d’utilisation** : Idéal pour les entreprises disposant d’une infrastructure SAML existante (Active Directory, Okta, etc.)
- **Complexité de configuration** : Plus impliquée — nécessite la configuration de l’IdP et la gestion des certificats
- **Fonctionnalités entreprise** : Mappage avancé des rôles, gestion centralisée des utilisateurs, pistes d’audit

### Quand choisir SAML

Envisagez l’authentification SAML si votre organisation :

- Utilise déjà un fournisseur d’identité compatible SAML (Okta, Azure AD, ADFS, etc.)
- Exige une sécurité et une conformité de niveau entreprise
- A besoin d’une gestion centralisée des utilisateurs et d’un contrôle d’accès
- Dispose de plusieurs applications utilisant SAML pour l’authentification
- Nécessite des pistes d’audit détaillées et des rapports de sécurité

### Quand choisir SSO simple ou sécurisé

Nos solutions SSO axées sur le widget peuvent être suffisantes si vous :

- Disposez d’un système d’authentification personnalisé
- Avez besoin d’une mise en œuvre rapide avec une configuration minimale
- Ne nécessitez pas l’intégration d’un fournisseur d’identité d’entreprise
- Souhaitez contrôler les données utilisateur directement depuis votre application
- Avez des exigences de sécurité plus simples

Le SSO simple et sécurisé est couramment utilisé pour des portails en ligne, des blogs, etc., où l’utilisateur possède déjà un compte *via votre site ou application*
mais n’utilise pas nécessairement SAML.