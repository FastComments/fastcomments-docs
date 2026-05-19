La intégration FastComments LTI 1.3 suit le principe du moindre privilège : elle n'utilise que les revendications de lancement requises pour identifier l'utilisateur, attacher les commentaires au bon cours et à la bonne ressource, et appliquer des autorisations basées sur les rôles.

Le reste de cette page cartographie chaque revendication que l'intégration consomme, chaque service LTI Advantage qu'elle ne demande pas, et chaque catégorie de données qu'elle ne collecte pas. Les évaluateurs de la sécurité et des achats peuvent extraire des réponses directement des tableaux ci-dessous.

## Éléments de données reçus du LMS

Chaque lancement LTI 1.3 contient un JWT signé provenant du LMS. FastComments extrait les revendications suivantes de ce JWT et n'utilise rien d'autre :

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifie de façon cohérente l'utilisateur entre les lancements afin que la même personne corresponde au même utilisateur SSO FastComments | Oui | Oui, comme partie d'un ID SSO interne stable |
| Display name | `name` | Attribution affichée à côté des commentaires de l'utilisateur | Oui (retombe sur "Utilisateur LMS" si absent) | Oui |
| Email | `email` | Correspondance de compte, notifications, modération, correspondance de support | Optionnel (l'intégration fonctionne sans) | Oui lorsqu'il est fourni |
| Avatar URL | `picture` | Affiché sur les commentaires de l'utilisateur | Optionnel | URL seulement ; FastComments ne télécharge pas et ne réhéberge pas l'image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Détermine si l'utilisateur est administrateur, instructeur (modérateur) ou apprenant | Oui | Drapeaux `isAdmin` / `isModerator` dérivés sur la session SSO |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associe le fil de commentaires au bon cours LMS | Oui | Oui, comme partie de l'identifiant de page résolu |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associe les commentaires à l'activité ou à l'emplacement de l'outil correct à l'intérieur du cours | Oui lorsqu'il est présent | Oui, comme partie de l'identifiant de page résolu |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Oriente le lancement vers la bonne configuration de locataire FastComments | Oui | Oui, dans l'enregistrement de configuration LTI FastComments |

## Revendications et étendues déclarées lors de l'enregistrement

Lors de l'enregistrement dynamique LTI 1.3, FastComments s'enregistre avec `scope: ""` (aucune étendue OAuth additionnelle) et déclare uniquement ces revendications OpenID Connect :

`iss`, `sub`, `name`, `email`, `picture`

Il enregistre deux types de messages :

- `LtiResourceLinkRequest` - le lancement de cours standard vers FastComments.
- `LtiDeepLinkingRequest` - permet aux instructeurs de placer l'outil FastComments à l'intérieur d'un cours.

Aucun jeton d'accès supplémentaire n'est demandé au LMS.

## Services LTI Advantage non demandés

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Services de provision des noms et des rôles (NRPS) | Non | L'intégration n'a pas besoin de la liste des participants du cours ; l'identité de l'utilisateur est fournie à chaque lancement |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Non | L'intégration n'est pas liée au carnet de notes |
| Deep Linking beyond the standard placement return | Aucune donnée supplémentaire | Le deep linking est utilisé uniquement pour le placement de l'outil par l'instructeur ; aucun contenu de cours n'est énuméré |

## Données non collectées

Outre LTI lui‑même, FastComments ne demande ni ne reçoit les éléments suivants du LMS ou de l'utilisateur :

| Catégorie | Collectée ? |
|----------|------------|
| Student grades | Non |
| Assignment submissions | Non |
| Attendance records | Non |
| Full course rosters | Non |
| Government identifiers | Non |
| Date of birth | Non |
| Postal address or phone number | Non |
| Financial information | Non |
| LMS administrator credentials | Non |

## Limites d'accès

- FastComments ne reçoit des données que dans le cadre d'un lancement LTI 1.3 autorisé signé par les clés enregistrées du LMS. L'intégration n'interroge pas le LMS pour obtenir des informations supplémentaires.
- Les jetons de lancement sont à usage unique et de courte durée. Les jetons rejoués ou expirés sont rejetés.
- Les administrateurs LMS contrôlent où l'outil est déployé dans leur plateforme. D2L Brightspace, par exemple, prend en charge le ciblage par unité organisationnelle et les paramètres de sécurité par déploiement, ce qui permet aux administrateurs de restreindre l'outil à des cours ou unités organisationnelles spécifiques plutôt que de le rendre disponible globalement. Moodle, Blackboard, Sakai et Schoology offrent des contrôles équivalents par déploiement dans leurs implémentations LTI 1.3.

## Stockage et conservation

FastComments conserve les données dérivées de LTI pendant la durée du service de commentaires actif et conformément aux paramètres de conservation configurés par le client. Les données de commentaires sont stockées dans un stockage de production chiffré au repos. Lors de la résiliation du compte ou sur demande écrite de suppression, FastComments supprime ou anonymise les données client conformément à l'accord applicable.

Pour les détails complets sur le stockage et le traitement des données, voir la <a href="https://fastcomments.com/privacy-policy" target="_blank">Politique de confidentialité de FastComments</a>.

## Fréquence de révision

Toute nouvelle fonctionnalité LTI nécessitant des revendications, des étendues ou des services LTI Advantage supplémentaires est examinée avant la publication afin de confirmer que l'accès demandé est nécessaire et proportionnel à la fonctionnalité livrée.

## Brève déclaration pour les questionnaires de sécurité

> FastComments applique le principe du moindre privilège et la minimisation des données à son intégration LTI 1.3. L'intégration n'utilise que les revendications de lancement LTI nécessaires pour authentifier l'utilisateur (`sub`, `name`, `email`, `picture`), déterminer son rôle et identifier le cours et la ressource auxquels les commentaires appartiennent. FastComments ne demande pas les services de provision des noms et des rôles (NRPS), les services d'affectation et de notation (AGS), les données du carnet de notes, la présence, les listes complètes de participants, ni l'accès administratif au LMS. Les administrateurs LMS conservent le contrôle sur les unités organisationnelles, les cours et les déploiements dans lesquels l'outil est disponible.