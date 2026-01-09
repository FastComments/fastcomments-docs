SAML (Security Assertion Markup Language) est une norme ouverte basée sur XML pour l'échange de données d'authentification et d'autorisation entre parties, 
particulièrement entre un fournisseur d'identité (IdP) et un fournisseur de services (SP).

### Comment fonctionne SAML

SAML permet l'authentification unique (SSO) en permettant aux utilisateurs de s'authentifier une seule fois auprès de leur fournisseur d'identité puis d'accéder à plusieurs applications 
sans ressaisir leurs informations d'identification. Lorsque un utilisateur tente d'accéder à FastComments :

1. **Authentication Request** : FastComments redirige l'utilisateur vers votre fournisseur d'identité
2. **User Authentication** : L'utilisateur s'authentifie auprès de votre IdP (par ex., Active Directory, Okta, Azure AD)
3. **SAML Response** : L'IdP envoie une assertion SAML signée à FastComments
4. **User Access** : FastComments valide l'assertion et accorde l'accès à l'utilisateur authentifié

### Avantages de SAML

- **Sécurité renforcée** : L'authentification centralisée réduit les risques liés aux mots de passe
- **Expérience utilisateur améliorée** : Les utilisateurs se connectent une seule fois et accèdent à plusieurs applications sans interruption
- **Conformité** : Aide à satisfaire les exigences réglementaires en matière de contrôle d'accès et de traces d'audit
- **Contrôle administratif** : Les administrateurs informatiques conservent une gestion centralisée des utilisateurs

### Prise en charge de SAML 2.0

FastComments implémente SAML 2.0, la version la plus largement adoptée de la norme SAML. Notre implémentation prend en charge :

- liaisons HTTP-POST et HTTP-Redirect
- réponses et assertions SAML signées
- assertions chiffrées (optionnelles)
- plusieurs algorithmes de signature et de hachage
- divers formats d'identificateur de nom