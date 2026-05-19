Une fois qu’un administrateur a enregistré FastComments en tant qu’outil LTI 1.3 Advantage et approuvé les politiques de l’établissement, les enseignants l’ajoutent aux cours via les points de placement standard de Blackboard. Les étapes exactes diffèrent entre Ultra Course View et Original Course View, les deux cas sont donc abordés ci-dessous.

#### Ultra Course View

Ultra Course View est l’affichage par défaut dans Blackboard Learn SaaS à partir de 2026.

1. Ouvrez le cours et allez sur la page **Course Content**.
2. Survolez ou touchez l’endroit où vous voulez que le fil de commentaires apparaisse dans le plan et cliquez sur le bouton violet **+** (Add content).
3. Choisissez **Content Market**. Le panneau Content Market répertorie tous les outils LTI approuvés et les placements Building Block pour votre établissement.
4. Trouvez la tuile **FastComments** et cliquez dessus. Blackboard crée un élément de contenu à l’emplacement où vous avez ouvert le menu **+**.
5. Par défaut, l’élément apparaît dans le plan en tant qu’entrée « Visible to students » pour les enseignants dont le paramètre personnel par défaut **Hide from students** est désactivé. Si votre défaut est **Hidden**, l’élément est créé en mode masqué et vous activez le sélecteur de visibilité sur la ligne de l’élément lorsque vous êtes prêt.
6. Pour renommer l’élément, cliquez sur le titre dans le plan et saisissez une nouvelle étiquette. Le titre que les étudiants voient dans le plan est indépendant de l’identifiant du fil FastComments, donc le renommage est sans risque à tout moment.

Si vous ne voyez pas **Content Market** comme option, votre établissement a masqué ce placement. Vous pouvez également accéder au même sélecteur via **More tools** dans le même menu **+** sous le groupe **LTI Tools**.

#### Original Course View

Original Course View est toujours pris en charge dans Learn SaaS et reste l’expérience principale pour les sites Learn 9.1 auto‑hébergés sur la ligne de version CU de Q4 2024.

1. Ouvrez le cours et entrez dans une **Content Area** (par exemple, la **Information** ou la zone **Content** par défaut dans le menu du cours).
2. Activez **Edit Mode** avec l’interrupteur en haut à droite de la page.
3. Cliquez sur **Build Content** dans la barre d’actions.
4. Sous le sous‑menu **Learning Tools**, cliquez sur **FastComments**. Le sous‑menu Learning Tools est rempli à partir des placements d’outils LTI 1.3 après qu’un administrateur a enregistré l’outil. Si vous ne le voyez pas, consultez la section des problèmes ci‑dessous.
5. Sur le formulaire **Create FastComments**, définissez :
   - **Name** : le libellé que voient les étudiants dans la zone de contenu.
   - **Description** : texte optionnel affiché au‑dessus du fil intégré.
   - **Permit Users to View this Content** : bascule de disponibilité Oui/Non.
   - **Track Number of Views** : activez si vous voulez les statistiques de consultations par élément de Blackboard. FastComments collecte ses propres analyses indépendamment.
   - **Date and Time Restrictions** : fenêtres optionnelles **Display After** / **Display Until**.
6. Soumettez. L’outil apparaît comme un élément cliquable dans la zone de contenu.

#### Embedding Inside an Item or Document

Dans les deux affichages de cours, les enseignants intègrent FastComments en ligne dans le corps d’un Item, d’un Document ou de n’importe quel champ riche via le bouton LTI Advantage de l’éditeur de contenu.

Ultra Course View :

1. Créez ou modifiez un **Document**.
2. Cliquez sur **Add content** à l’intérieur du corps du document à l’endroit où vous voulez que le fil apparaisse.
3. Dans la barre d’outils de l’éditeur, ouvrez le menu **Insert content** et cliquez sur **Content Market** (le point d’entrée LTI Advantage / Deep Linking).
4. Choisissez **FastComments**. FastComments renvoie une charge utile de deep‑link et Blackboard insère un bloc embarqué dans le corps du document à la position du curseur.
5. Enregistrez le document. Les étudiants voient le fil rendu en ligne lorsqu’ils font défiler la page.

Original Course View :

1. Modifiez n’importe quel élément avec un corps en texte enrichi.
2. Dans la barre d’outils de l’éditeur de contenu, cliquez sur l’icône plus **Add Content** et choisissez **Content Market** (étiqueté **Add Content from External Tool** dans les anciennes CU de Q4 2024).
3. Choisissez **FastComments**. L’éditeur insère un bloc de remplacement faisant référence à la ressource deep‑linked.
4. Soumettez l’élément.

Chaque intégration deep‑link crée son propre fil FastComments, donc un Item avec deux blocs FastComments intégrés possède deux flux de commentaires indépendants.

#### Visibility, Release Conditions, and Group Restrictions

Les éléments de contenu FastComments se comportent comme n’importe quel autre élément de contenu Blackboard pour ce qui est des règles de contrôle d’accès qui leur sont appliquées.

- Ultra : cliquez sur le sélecteur de visibilité sur la ligne (**Visible to students**, **Hidden from students**, **Conditional availability**). La Conditional availability prend en charge les fenêtres date/heure, les règles de performance basées sur des éléments du carnet de notes, et les règles de membre basées sur les groupes du cours.
- Original : ouvrez le menu contextuel de l’élément et choisissez **Adaptive Release** ou **Adaptive Release: Advanced** pour restreindre l’accès à l’outil par date, appartenance, note ou statut de revue. Utilisez **Set Group Availability** sur l’élément pour le restreindre à des groupes de cours spécifiques.

FastComments respecte la décision de verrouillage de Blackboard. Si Blackboard masque l’élément pour un étudiant, le lancement LTI n’a jamais lieu pour cet étudiant et il n’apparaît pas dans la vue du modérateur.

#### Gradebook Behavior

FastComments ne renvoie pas de notes via LTI Advantage Assignment and Grade Services. Aucune colonne de notes n’est créée automatiquement pour les éléments de contenu FastComments.

Si votre locataire Blackboard est configuré pour créer automatiquement une colonne dans le carnet de notes pour chaque nouvel élément de contenu indépendamment des métadonnées de notation, une colonne vide apparaît quand même. Pour la masquer :

- Ultra : ouvrez le **Gradebook**, cliquez sur l’en‑tête de colonne, choisissez **Edit**, et désactivez **Show to students** ainsi que **Include in calculations**. Ou utilisez **Delete** si votre établissement autorise la suppression de colonnes pour les éléments non notés.
- Original : ouvrez le **Grade Center**, cliquez sur le chevron de la colonne, choisissez **Hide from Users (on/off)**, et éventuellement **Hide from Instructor View** sous **Column Organization**.

#### What Students See

Lorsque un étudiant ouvre l’élément FastComments ou fait défiler jusqu’à un bloc intégré :

1. Blackboard lance le message LTI 1.3 vers FastComments. L’étudiant est authentifié via SSO en utilisant son identité Blackboard (nom, email, avatar, rôle) sans voir de formulaire de connexion.
2. Le fil de commentaires se rend dans l’iframe. Le fil, les réponses imbriquées, les mentions et les réactions sont tous disponibles en fonction des paramètres du widget de commentaires configurés dans FastComments.
3. Leurs commentaires sont attribués à leur compte Blackboard. Si l’étudiant modifie son nom ou sa photo dans Blackboard par la suite, le lancement suivant mettra à jour le profil FastComments.

Mappage des rôles de Blackboard vers FastComments :

- **System Administrator** et **Course Builder** mappent sur FastComments **admin**.
- **Instructor** et **Teaching Assistant** mappent sur FastComments **moderator**.
- **Student**, **Guest**, et **Observer** mappent sur FastComments **commenter**.

Les modérateurs voient les contrôles de modération (épingler, masquer, bannir, supprimer) inline sur chaque commentaire du fil.

#### Lock Down Public Access (Recommended)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Toute personne capable de deviner l’URL du fil ou le point de terminaison API peut voir ses commentaires, même en dehors de Blackboard. Pour les discussions de cours, vous souhaiterez presque certainement restreindre la consultation aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> et créez une règle avec **Require SSO To View Comments** activé, puis réglez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou une seule page.

#### Thread Scoping

FastComments scope chaque fil par **(Blackboard host, course ID, resource link ID)**. Deux éléments FastComments dans le même cours produisent deux fils. Le même élément copié dans deux coques de cours différentes (par exemple via la copie de cours) produit deux fils, parce que Blackboard émet un nouveau resource link ID lors de la copie. Pour conserver un fil partagé lors des copies de cours, utilisez Deep Linking avec un URN de fil explicite configuré dans FastComments avant d’exécuter la copie.

#### Problèmes spécifiques à Blackboard

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** L’administrateur a approuvé l’outil mais a laissé une politique d’établissement bloquant le placement pertinent. Allez dans **Administrator Panel** > **Integrations** > **LTI Tool Providers**, éditez l’entrée FastComments, et confirmez que les placements **Course Content Tool** (Original) et **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) sont activés. Enregistrez et actualisez la page du cours.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** La portée de déploiement enregistrée lors de l’enregistrement dynamique ne correspond pas au contexte de l’établissement auquel appartient le cours. Dans l’entrée du fournisseur d’outils de Blackboard, vérifiez que le **Deployment ID** correspond à ce que FastComments affiche sur sa page LTI 1.3 Configuration pour ce locataire. S’ils diffèrent, supprimez le placement et relancez l’enregistrement dynamique à partir d’une URL d’enregistrement fraîche (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**Iframe height looks fixed or content gets cut off.** Certains locataires Blackboard expédient une Content Security Policy stricte qui bloque le postMessage de redimensionnement d’iframe LTI par défaut. FastComments émet à la fois le message style Canvas `lti.frameResize` et le message conforme à la spec IMS `org.imsglobal.lti.frameResize` pour maximiser la compatibilité, mais une substitution CSP au niveau du locataire bloque le listener parent. Demandez à votre administrateur de confirmer que `*.fastcomments.com` est sur la liste blanche des outils LTI et qu’aucun en‑tête CSP personnalisé ne supprime les événements postMessage. Le redimensionnement fonctionnera alors sans configuration supplémentaire.

**Course copy duplicates threads.** La copie de cours Blackboard émet de nouveaux resource link IDs pour les placements LTI, donc les cours copiés commencent avec des fils vides. C’est attendu. Si vous avez besoin que le cours copié hérite du fil d’origine, configurez le Deep Linking avec un URN de fil explicite avant la copie, ou contactez le support FastComments pour remapper les IDs de fil en masse.

**Student sees a generic Blackboard error on launch.** La cause est une réclamation `email` manquante ou obsolète. Confirmez que la politique d’établissement pour FastComments a **Role**, **Name**, et **Email Address** activés sous **User Fields to Send**. Enregistrez, puis relancez dans une nouvelle session de navigateur.