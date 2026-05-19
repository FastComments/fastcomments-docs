L'intégration FastComments LTI 1.3 suit le principe du moindre privilège : elle utilise uniquement les claims de lancement nécessaires pour identifier l'utilisateur, associer les commentaires au cours et à la ressource corrects, et appliquer des permissions basées sur les rôles.

Le reste de cette page recense chaque claim que l'intégration consomme, chaque service LTI Advantage qu'elle ne demande pas, et chaque catégorie de données qu'elle ne collecte pas. Les examinateurs en sécurité et achats peuvent extraire les réponses directement des tableaux ci-dessous.

## Éléments de données reçus depuis le LMS

Chaque lancement LTI 1.3 contient un JWT signé provenant du LMS. FastComments extrait les claims suivants de ce JWT et n'utilise rien d'autre :

| Champ | LTI claim | Finalité | Obligatoire | Stocké |
|-------|-----------|---------|----------|--------|
| Identifiant utilisateur | `sub` | Identifie l'utilisateur de manière cohérente entre les lancements afin que la même personne corresponde au même utilisateur SSO FastComments | Oui | Oui, dans le cadre d'un identifiant SSO interne stable |
| Nom affiché | `name` | Attribution affichée à côté des commentaires de l'utilisateur | Oui (retour à "Utilisateur LMS" si absent) | Oui |
| Email | `email` | Appariement de compte, notifications, modération, correspondance support | Optionnel (l'intégration fonctionne sans) | Oui si fourni |
| URL d'avatar | `picture` | Affiché sur les commentaires de l'utilisateur | Optionnel | Seulement l'URL ; FastComments ne télécharge pas et ne réhéberge pas l'image |
| Rôles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Détermine si l'utilisateur est administrateur, enseignant (modérateur) ou apprenant | Oui | Drapeaux dérivés `isAdmin` / `isModerator` sur la session SSO |
| Contexte de cours | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associe le fil de commentaires au bon cours du LMS | Oui | Oui, en tant que partie de l'identifiant de page résolu |
| Lien de ressource | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associe les commentaires à l'activité ou à l'emplacement de l'outil correct à l'intérieur du cours | Oui lorsqu'il est présent | Oui, en tant que partie de l'identifiant de page résolu |
| ID de déploiement | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Oriente le lancement vers la configuration de locataire FastComments correcte | Oui | Oui, sur l'enregistrement de configuration LTI FastComments |

## Claims et scopes déclarés lors de l'enregistrement

Lors de l'enregistrement dynamique LTI 1.3, FastComments s'enregistre avec `scope: ""` (aucune étendue OAuth supplémentaire) et déclare uniquement ces claims OpenID Connect :

`iss`, `sub`, `name`, `email`, `picture`

Il enregistre deux types de messages :

- `LtiResourceLinkRequest` - le lancement standard de cours vers FastComments.
- `LtiDeepLinkingRequest` - permet aux enseignants de placer l'outil FastComments à l'intérieur d'un cours.

Aucun jeton d'accès supplémentaire n'est demandé au LMS.

## Services LTI Advantage non demandés

| Service / scope | Demandé ? | Raison |
|------------------|------------|--------|
| Services de provision des noms et des rôles (NRPS) | Non | L'intégration n'a pas besoin d'une liste de cours ; l'identité de l'utilisateur arrive avec chaque lancement |
| Services de devoirs et de notes (AGS) - lineitem, score, result scopes | Non | L'intégration n'est pas connectée au carnet de notes |
| Deep Linking au-delà du retour de placement standard | Aucune donnée supplémentaire | Le deep linking est utilisé uniquement pour le placement de l'outil par l'enseignant ; aucun contenu de cours n'est énuméré |

## Données non collectées

Au-delà du LTI lui-même, FastComments ne demande ni ne reçoit les éléments suivants du LMS ou de l'utilisateur :

| Catégorie | Collectée ? |
|----------|------------|
| Notes des étudiants | Non |
| Soumissions de devoirs | Non |
| Données d'assiduité | Non |
| Listes complètes des inscrits | Non |
| Identifiants gouvernementaux | Non |
| Date de naissance | Non |
| Adresse postale ou numéro de téléphone | Non |
| Informations financières | Non |
| Identifiants d'administrateur LMS | Non |

## Limites d'accès

- FastComments ne reçoit des données que dans le cadre d'un lancement LTI 1.3 autorisé signé par les clés enregistrées du LMS. L'intégration ne rappelle pas le LMS pour obtenir des informations supplémentaires.
- Les jetons de lancement sont à usage unique et de courte durée. Les jetons rejoués ou expirés sont refusés.
- Les administrateurs LMS contrôlent où l'outil est déployé dans leur plateforme. D2L Brightspace, par exemple, prend en charge le cloisonnement par unité organisationnelle par déploiement et des paramètres de sécurité par déploiement, ce qui permet aux administrateurs de restreindre l'outil à des cours ou unités organisationnelles spécifiques plutôt que de le rendre disponible globalement. Moodle, Blackboard, Sakai et Schoology offrent des contrôles équivalents par déploiement dans leurs implémentations LTI 1.3.

## Stockage et conservation

FastComments conserve les données dérivées du LTI pendant la durée du service de commentaires actif et conformément aux paramètres de conservation configurés par le client. Les données de commentaires sont stockées dans un stockage de production chiffré au repos. En cas de résiliation de compte ou de demande écrite de suppression, FastComments supprime ou anonymise les données clients conformément à l'accord applicable.

Pour les détails complets sur le stockage et le traitement des données, voir la <a href="https://fastcomments.com/privacy-policy" target="_blank">Politique de confidentialité FastComments</a>.

## Fréquence de révision

Toute nouvelle fonctionnalité LTI nécessitant des claims, des scopes ou des services LTI Advantage supplémentaires est examinée avant sa publication pour confirmer que l'accès demandé est nécessaire et proportionné à la fonctionnalité fournie.

## Brève déclaration pour les questionnaires de sécurité

> FastComments applique le principe du moindre privilège et la minimisation des données à son intégration LTI 1.3. L'intégration utilise uniquement les claims de lancement LTI nécessaires pour authentifier l'utilisateur (`sub`, `name`, `email`, `picture`), déterminer son rôle et identifier le cours et la ressource auxquels les commentaires appartiennent. FastComments ne demande ni les Services de provision des noms et des rôles, ni les Services de devoirs et de notes, ni les données du carnet de notes, ni les données d'assiduité, ni les listes complètes des inscrits, ni l'accès administratif au LMS. Les administrateurs LMS conservent le contrôle sur les unités organisationnelles, les cours et les déploiements dans lesquels l'outil est disponible.