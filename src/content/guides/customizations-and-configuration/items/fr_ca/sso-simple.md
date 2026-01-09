[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Avec Simple SSO, nous pouvons fournir au widget de commentaires des informations sur l'utilisateur afin qu'il n'ait pas à saisir son nom d'utilisateur ou son courriel pour commenter.

Nous pouvons configurer Simple SSO comme suit :

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

L'utilisateur sera connecté et un utilisateur SSO sera créé en arrière-plan. L'utilisateur aura `createdFromSimpleSSO` défini sur `true` s'il est récupéré via l'API.

Notes: 

- Le courriel est l'identifiant unique pour Simple SSO.
- Fournir un courriel avec Simple SSO n'est pas obligatoire, cependant, par défaut, leurs commentaires s'afficheront comme "Non vérifié". <b>Si aucun courriel n'est fourni, l'utilisateur ne peut pas être pleinement authentifié.</b>
- **NOUVEAU** Depuis janv. 2022 : les noms d'utilisateur n'ont pas à être uniques sur l'ensemble de fastcomments.com
- Simple SSO peut créer et mettre à jour automatiquement des utilisateurs SSO, si un courriel est fourni, et si l'utilisateur n'a pas été créé à l'origine via Secure SSO.
- Vous pouvez spécifier des badges pour l'utilisateur avec la propriété `badgeConfig`. Le tableau `badgeIds` contient les IDs des badges à associer à l'utilisateur. Si `override` est défini sur `true`, il remplacera tous les badges existants affichés sur les commentaires ; si `false`, il s'ajoutera aux badges existants.