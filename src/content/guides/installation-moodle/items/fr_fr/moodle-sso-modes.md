---
Le plugin prend en charge trois modes SSO pour l'intégration des comptes utilisateurs Moodle avec FastComments.

#### SSO sécurisé (recommandé)

Les données utilisateur sont signées côté serveur en utilisant HMAC-SHA256 avec votre API Secret. Il s'agit de l'option la plus sécurisée et elle est recommandée pour une utilisation en production.

Avec le SSO sécurisé :

- Les noms d'utilisateur, les e-mails et les avatars sont transmis de manière sécurisée à FastComments.
- Les administrateurs du site Moodle sont automatiquement synchronisés en tant que modérateurs FastComments.
- Les utilisateurs ne peuvent pas usurper d'autres comptes.
- Nécessite que le **API Secret** soit configuré dans les paramètres du plugin.

#### SSO simple

Les données utilisateur (nom, e-mail, avatar) sont envoyées côté client sans signature cryptographique. Cela est plus facile à configurer car cela ne nécessite pas d'API Secret, mais c'est moins sécurisé car les données utilisateur ne sont pas vérifiées côté serveur.

#### Aucun

Aucune intégration SSO. Les utilisateurs commentent anonymement ou doivent se connecter séparément à FastComments. Utilisez ceci si vous ne souhaitez pas lier les comptes utilisateurs Moodle aux commentaires.

---