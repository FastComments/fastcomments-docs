Le plugin prend en charge trois modes SSO pour l'intégration des comptes utilisateurs Moodle avec FastComments.

#### SSO sécurisé (Recommandé)

Les données utilisateur sont signées côté serveur à l'aide de HMAC-SHA256 avec votre API Secret. Il s'agit de l'option la plus sécurisée et elle est recommandée pour une utilisation en production.

Avec le SSO sécurisé :

- Les noms d'utilisateurs, les courriels et les avatars sont transmis de façon sécurisée à FastComments.
- Les administrateurs du site Moodle sont synchronisés automatiquement en tant que modérateurs FastComments.
- Les utilisateurs ne peuvent pas usurper d'autres comptes.
- Nécessite que le **API Secret** soit configuré dans les paramètres du plugin.

#### SSO simple

Les données utilisateur (nom, courriel, avatar) sont envoyées côté client sans signature cryptographique. Cela est plus facile à configurer puisqu'il ne nécessite pas d'API Secret, mais c'est moins sécurisé parce que les données utilisateur ne sont pas vérifiées côté serveur.

#### Aucun

Aucune intégration SSO. Les utilisateurs commentent anonymement ou doivent se connecter à FastComments séparément. Utilisez ceci si vous ne voulez pas que les comptes d'utilisateurs Moodle soient liés aux commentaires.