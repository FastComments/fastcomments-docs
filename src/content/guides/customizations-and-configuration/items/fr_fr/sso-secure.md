[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilise le chiffrement HMAC-SHA256 comme mécanisme pour implémenter le SSO. Nous allons d'abord passer en revue l'architecture globale, fournir des exemples et des étapes détaillées.

Il existe également une documentation concernant la migration depuis d'autres fournisseurs utilisant des mécanismes SSO similaires, ainsi que les différences.

Le flux ressemble à ceci :

<div class="screenshot white-bg">
    <div class="title">Flux SSO sécurisé</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Schéma SSO sécurisé" />
</div>

Étant donné que Secure SSO implique un développement full-stack, des exemples de code entièrement fonctionnels en Java/Spring, NodeJS/Express et PHP natif sont actuellement <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">sur GitHub</a>.

Bien que nous utilisions ExpressJS dans l'exemple NodeJS et Spring dans l'exemple Java, aucun framework/bibliothèque n'est requis dans ces environnements pour implémenter FastComments SSO — les packages crypto natifs suffisent.

Vous n'avez pas besoin d'écrire de nouveaux endpoints API avec FastComments SSO. Il suffit de chiffrer les informations de l'utilisateur en utilisant votre clé secrète et de transmettre la payload au widget de commentaires.

#### Récupérez votre clé secrète API

Votre clé secrète API peut être récupérée depuis <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">cette page</a>. Vous pouvez également accéder à cette page en allant dans Mon compte, en cliquant sur la tuile API/SSO, puis en cliquant sur "Get API Secret Key".

#### Paramètres du widget de commentaires

La documentation API de haut niveau pour le widget de commentaires se trouve <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ici</a>.

Voyons plus en détail ce que signifient ces paramètres.

Le widget de commentaires prend un objet de configuration — vous passez déjà cet objet si vous utilisez FastComments pour fournir votre tenantId.

Pour activer le SSO, passez un nouvel objet "sso", qui doit contenir les paramètres suivants. Les valeurs doivent être générées côté serveur.

- userDataJSONBase64: The user's data in JSON format, which is then Base64 encoded.
- verificationHash: The HMAC-SHA256 hash created from UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Must not be in the future, or more than two days in the past.
- loginURL: A URL that the comment widget can show to log the user in.
- logoutURL: A URL that the comment widget can show to log the user out.
- loginCallback: When provided instead of the login URL, a function that the comment widget will invoke when clicking the login button.
- logoutCallback: When provided instead of the logout URL, a function that the comment widget will invoke when clicking the logout button.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objet utilisateur

The User object contains the following schema:
[inline-code-attrs-start title = 'Objet utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Requis. 1k caractères max. **/
    id: string;
    /** Requis. 1k caractères max. Remarque : Doit être unique. **/
    email: string;
    /** Requis. 1k caractères max. Remarque : Le nom d'utilisateur ne peut pas être un e-mail. N'a pas besoin d'être unique. **/
    username: string;
    /** Optionnel. 3k caractères max pour les URLs. Par défaut, provient de gravatar en fonction de l'e-mail. Prend en charge les images encodées en base64, auquel cas la limite est de 50k caractères. **/ 
    avatar?: string;
    /** Optionnel. Par défaut false. **/
    optedInNotifications?: boolean;
    /** Optionnel. Par défaut false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optionnel. 100 caractères max. Ce label sera affiché à côté de leur nom. Par défaut, Administrateur/Modérateur le cas échéant. **/
    displayLabel?: string;
    /** Optionnel. 500 caractères max. Ceci sera affiché à la place du nom d'utilisateur. **/
    displayName?: string;
    /** Optionnel. 2k caractères max. Le nom de l'utilisateur renverra vers cette URL. **/
    websiteUrl?: string;
    /** Optionnel. Jusqu'à 100 groupes par utilisateur. Un id de groupe ne peut pas dépasser 50 caractères. **/
    groupIds?: string[];
    /** Optionnel. Désigne l'utilisateur comme administrateur. **/
    isAdmin?: boolean;
    /** Optionnel. Désigne l'utilisateur comme modérateur. **/
    isModerator?: boolean;
    /** Optionnel, par défaut true. Mettre à false pour activer l'onglet "activity" dans le profil de l'utilisateur. **/
    isProfileActivityPrivate?: boolean;
    /** Optionnel, par défaut false. Mettre à true pour désactiver les commentaires du profil. **/
    isProfileCommentsPrivate?: boolean;
    /** Optionnel, par défaut false. Mettre à true pour désactiver la messagerie directe avec cet utilisateur. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Modérateurs et administrateurs

Pour les admins et modérateurs, passez les indicateurs `isAdmin` ou `isModerator` respectifs dans l'objet `SSOUser`.

#### Notifications

Pour activer ou désactiver les notifications, définissez la valeur de `optedInNotifications` sur `true` ou `false` respectivement. La première fois que l'utilisateur charge la page avec cette valeur dans la payload SSO, ses paramètres de notification seront mis à jour.

De plus, si vous souhaitez que les utilisateurs reçoivent des emails de notification pour l'activité sur les pages auxquelles ils sont abonnés (plutôt que seulement des notifications dans l'application), définissez `optedInSubscriptionNotifications` sur `true`.

#### Utilisateurs VIP et labels spéciaux

Vous pouvez afficher un label spécial à côté du nom de l'utilisateur en utilisant le champ optionnel "displayLabel".

#### Utilisateurs non authentifiés

Pour représenter un utilisateur non authentifié, ne remplissez simplement pas userDataJSONBase64, verificationHash ou timestamp. Fournissez un loginURL.

Ces utilisateurs ne pourront pas commenter et se verront présenter un message de connexion (message, lien ou bouton, selon la configuration).

#### Exemples directs de sérialisation et de hachage des données utilisateur

Plus de détails et des exemples sont disponibles <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ici</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ici</a> (java) et <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ici</a> (php).

Nous comprenons que toute intégration peut être un processus compliqué et pénible. N'hésitez pas à contacter votre représentant ou à utiliser la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">page d'assistance</a>.