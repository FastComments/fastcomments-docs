[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Avec Simple SSO, nous pouvons fournir au widget de commentaires des informations sur l'utilisateur afin qu'il n'ait pas à saisir son nom d'utilisateur ou son e‑mail pour commenter.

Nous pouvons configurer Simple SSO comme suit :

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

L'utilisateur sera connecté et un utilisateur SSO sera créé en arrière‑plan. L'utilisateur aura `createdFromSimpleSSO` défini sur `true` s'il est récupéré via l'API.

Remarques :

- L'e‑mail est l'identifiant unique pour Simple SSO.
- Fournir un e‑mail avec Simple SSO n'est pas obligatoire, cependant par défaut leurs commentaires apparaissent comme « Non vérifié ». <b>Si aucun e‑mail n'est fourni, l'utilisateur ne peut pas être entièrement authentifié.</b>
- **NOUVEAU** Depuis janvier 2022 : les noms d'utilisateur n'ont pas besoin d'être uniques sur l'ensemble de fastcomments.com
- Simple SSO peut créer et mettre à jour automatiquement les utilisateurs SSO, si un e‑mail est fourni, et que l'utilisateur n'a pas été créé à l'origine via Secure SSO. Si une charge Simple SSO désigne un utilisateur créé par Secure SSO ou l'API, cet utilisateur reste inchangé et le visiteur obtient un utilisateur distinct.
- Vous pouvez spécifier des badges pour l'utilisateur avec la propriété `badgeConfig`. Le tableau `badgeIds` contient les identifiants des badges globaux à associer à l'utilisateur. Le tableau `pageBadgeIds` contient les identifiants de badges limités à la page actuelle (`urlId`) — ces badges ne sont affichés que sur la page où ils ont été assignés. Si `override` est défini sur `true`, il remplacera les badges affichés existants (les badges globaux et ceux limités à la page sont remplacés indépendamment) ; si `false`, il s'ajoutera aux badges existants.

#### Paramètres Simple SSO

Simple SSO est toujours disponible. Deux paramètres contrôlent le niveau de confiance que FastComments lui accorde. Vous pouvez les trouver en recherchant « SSO » dans une <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">règle de personnalisation</a>. Les deux sont désactivés par défaut, et s'appliquent que la règle « Exiger SSO pour voir les commentaires » soit activée ou non.

Les comptes qui utilisaient déjà Simple SSO lorsque ces paramètres ont été ajoutés ont les deux activés, donc rien n'a changé pour eux.

Seule la règle de personnalisation qui correspond le mieux à une page s'applique, donc si vous avez plusieurs règles, définissez-les sur chacune d'elles. Une nouvelle règle commence avec ces paramètres activés si l'une de vos autres règles les a activés.

##### Vérification automatique des commentaires via Simple SSO

Lorsque ce paramètre est activé, les commentaires des utilisateurs Simple SSO sont vérifiés immédiatement.

Lorsque ce paramètre est désactivé, les commentaires des utilisateurs Simple SSO apparaissent comme « Non vérifié » jusqu'à ce que l'utilisateur clique sur un lien que nous envoyons par e‑mail à l'adresse contenue dans la charge Simple SSO. Si vous n'approuvez automatiquement que les commentaires vérifiés, ils attendent également ce lien. S'il n'y a pas d'e‑mail dans la charge, le commentaire reste non vérifié.

Ce paramètre n'affecte que les commentaires. Les votes des utilisateurs Simple SSO sont toujours comptés immédiatement.

##### Autoriser les administrateurs Simple SSO

Lorsque ce paramètre est activé, un utilisateur Simple SSO peut être administrateur ou modérateur de votre compte :

- Un utilisateur Simple SSO qui possède des permissions d'administrateur ou de modérateur les conserve.
- Un utilisateur Simple SSO ayant le même e‑mail qu'un de vos administrateurs ou modérateurs hérite de leurs permissions.

Lorsque ce paramètre est désactivé, Simple SSO n'accorde pas ces permissions. Une charge Simple SSO désignant un utilisateur qui les possède crée un utilisateur distinct sans permissions, et l'utilisateur original reste inchangé.

Les charges Simple SSO ne sont pas signées, donc lorsque ce paramètre est activé, toute personne connaissant l'e‑mail ou l'ID d'un administrateur peut se faire passer pour lui. Si cela pose problème pour votre site, laissez ce paramètre désactivé et utilisez Secure SSO pour vos administrateurs.

Les utilisateurs créés par Secure SSO ou l'API ne peuvent pas être atteints par Simple SSO, que ce paramètre soit activé ou non.

---