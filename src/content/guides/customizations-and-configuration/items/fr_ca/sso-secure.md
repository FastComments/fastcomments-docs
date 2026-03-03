[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilise le chiffrement HMAC-SHA256 comme mécanisme pour mettre en œuvre SSO. Nous commencerons par présenter l'architecture globale, fournir des exemples et des étapes détaillées.

Il existe également de la documentation concernant la migration depuis d'autres fournisseurs ayant des mécanismes SSO similaires, et les différences.

Le flux ressemble à ceci :

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Étant donné que Secure SSO implique du développement full-stack, des exemples de code complets et fonctionnels en Java/Spring, NodeJS/Express et PHP natif se trouvent actuellement <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">sur GitHub</a>.

Bien que nous utilisions ExpressJS dans l'exemple NodeJS et Spring dans l'exemple Java, aucun framework/bibliothèque n'est requis dans ces environnements pour implémenter FastComments SSO — les packages crypto natifs fonctionnent.

Vous n'avez pas besoin d'écrire de nouveaux endpoints API avec FastComments SSO. Il suffit de chiffrer les informations de l'utilisateur en utilisant votre clé secrète et de transmettre la charge utile au widget de commentaires.

#### Get Your API Secret Key

Votre clé secrète API peut être récupérée depuis <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">cette page</a>. Vous pouvez aussi accéder à cette page en allant dans My Account, en cliquant sur la tuile API/SSO, puis en cliquant sur "Get API Secret Key".

#### Comment Widget Parameters

La documentation API de haut niveau pour le widget de commentaires se trouve <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ici</a>.

Voyons plus en détail ce que signifient ces paramètres.

Le widget de commentaires prend un objet de configuration — vous fournissez déjà ceci si vous utilisez FastComments pour transmettre votre identifiant client (appelé tenantId).

Pour activer le SSO, passez un nouvel objet "sso", qui doit contenir les paramètres suivants. Les valeurs doivent être générées côté serveur.

- userDataJSONBase64: The user's data in JSON format, which is then Base64 encoded.
- verificationHash: The HMAC-SHA256 hash created from UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Must not be in the future, or more than two days in the past.
- loginURL: A URL that the comment widget can show to log the user in.
- logoutURL: A URL that the comment widget can show to log the user out.
- loginCallback: When provided instead of loginURL, a function that the comment widget will invoke when clicking the login button.
- logoutCallback: When provided instead of logoutURL, a function that the comment widget will invoke when clicking the logout button.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

L'objet utilisateur contient le schéma suivant :
[inline-code-attrs-start title = 'Objet utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Requis. Maximum 1k caractères. **/
    id: string;
    /** Requis. Maximum 1k caractères. Note : Doit être unique. **/
    email: string;
    /** Requis. Maximum 1k caractères. Remarque : Le nom d'utilisateur ne peut pas être une adresse courriel. N'a pas besoin d'être unique. **/
    username: string;
    /** Optionnel. Maximum 3k caractères pour les URL. Par défaut, provient de Gravatar basé sur le courriel. Prend en charge les images encodées en 64, auquel cas la limite est de 50k caractères. **/ 
    avatar?: string;
    /** Optionnel. Par défaut false. **/
    optedInNotifications?: boolean;
    /** Optionnel. Par défaut false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optionnel. Maximum 100 caractères. Cette étiquette sera affichée à côté de leur nom. Par défaut Administrateur/Modérateur lorsque applicable. **/
    displayLabel?: string;
    /** Optionnel. Maximum 500 caractères. Sera affiché à la place du nom d'utilisateur. **/
    displayName?: string;
    /** Optionnel. Maximum 2k caractères. Le nom de l'utilisateur renverra vers ceci. **/
    websiteUrl?: string;
    /** Optionnel. Jusqu'à 100 groupes par utilisateur. Un id de groupe ne peut pas dépasser 50 caractères. **/
    groupIds?: string[];
    /** Optionnel. Désigne l'utilisateur comme administrateur. **/
    isAdmin?: boolean;
    /** Optionnel. Désigne l'utilisateur comme modérateur. **/
    isModerator?: boolean;
    /** Optionnel, par défaut true. Mettez à false pour activer l'onglet "activity" dans le profil de l'utilisateur. **/
    isProfileActivityPrivate?: boolean;
    /** Optionnel, par défaut false. Mettez à true pour désactiver les commentaires du profil. **/
    isProfileCommentsPrivate?: boolean;
    /** Optionnel, par défaut false. Mettez à true pour désactiver la messagerie directe pour cet utilisateur. **/
    isProfileDMDisabled?: boolean;
    /** Configuration optionnelle pour les insignes utilisateur. **/
    badgeConfig?: {
        /** Tableau d'IDs d'insignes globaux à attribuer. Limité à 30 insignes. L'ordre est respecté. **/
        badgeIds: string[];
        /** Tableau d'IDs d'insignes limité à la page courante (urlId). Affichés uniquement sur la page assignée. **/
        pageBadgeIds?: string[];
        /** Si true, remplace les insignes affichés existants. Les insignes globaux et ceux spécifiques à une page sont remplacés indépendamment. **/
        override?: boolean;
        /** Si true, met à jour les propriétés d'affichage des insignes à partir de la configuration du tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Pour les administrateurs et les modérateurs, passez les flags respectifs `isAdmin` ou `isModerator` dans l'objet `SSOUser`.

#### Notifications

Pour activer ou désactiver les notifications, réglez la valeur de `optedInNotifications` sur `true` ou `false` respectivement. La première fois que l'utilisateur charge la page avec cette valeur dans la charge utile SSO, ses paramètres de notification seront mis à jour.

De plus, si vous voulez que les utilisateurs reçoivent des courriels de notification pour l'activité sur les pages auxquelles ils se sont abonnés (plutôt que seulement des notifications dans l'application), alors réglez `optedInSubscriptionNotifications` sur `true`.

#### VIP Users & Special Labels

Vous pouvez afficher une étiquette spéciale à côté du nom de l'utilisateur en utilisant le champ optionnel "displayLabel".

#### Unauthenticated users

Pour représenter un utilisateur non authentifié, il suffit de ne pas remplir userDataJSONBase64, verificationHash ou timestamp. Fournissez une loginURL.

Ces utilisateurs ne pourront pas commenter et verront plutôt un message de connexion (message, lien ou bouton, selon la configuration).

#### Direct Examples for Serializing and Hashing User Data

Plus de détails et des exemples sont disponibles <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ici</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ici</a> (java) et <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ici</a> (php).

Nous comprenons que toute intégration peut être un processus compliqué et éprouvant. N'hésitez pas à contacter votre représentant ou à utiliser la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">page de support</a>.

---