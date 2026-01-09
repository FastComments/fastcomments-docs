SAML (Security Assertion Markup Language) est une norme ouverte basée sur XML pour l'échange de données d'authentification et d'autorisation entre parties, 
particularly between an identity provider (IdP) and a service provider (SP).

### Comment fonctionne SAML

SAML permet le single sign-on (SSO) en permettant aux utilisateurs de s'authentifier une seule fois auprès de leur identity provider puis d'accéder à plusieurs applications 
sans ressaisir leurs identifiants. Lorsqu'un utilisateur tente d'accéder à FastComments:

1. **Demande d'authentification**: FastComments redirige l'utilisateur vers votre identity provider
2. **Authentification de l'utilisateur**: L'utilisateur s'authentifie auprès de votre IdP (par ex., Active Directory, Okta, Azure AD)
3. **Réponse SAML**: L'IdP envoie une assertion SAML signée à FastComments
4. **Accès utilisateur**: FastComments valide l'assertion et accorde l'accès à l'utilisateur authentifié

### Avantages de SAML

- **Sécurité renforcée**: L'authentification centralisée réduit les risques liés aux mots de passe
- **Expérience utilisateur améliorée**: Les utilisateurs se connectent une seule fois et accèdent à plusieurs applications de manière transparente
- **Conformité**: Aide à satisfaire les exigences réglementaires en matière de contrôle d'accès et de pistes d'audit
- **Contrôle administratif**: Les administrateurs IT maintiennent une gestion centralisée des utilisateurs

### Prise en charge de SAML 2.0

FastComments implémente SAML 2.0, la version du standard SAML la plus largement adoptée. Notre implémentation prend en charge:

- HTTP-POST and HTTP-Redirect bindings
- Réponses et assertions SAML signées
- Assertions chiffrées (optionnel)
- Plusieurs algorithmes de signature et de digest
- Divers formats de name identifier