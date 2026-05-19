Une fois qu’un administrateur a enregistré FastComments en tant qu’outil LTI 1.3 Advantage et approuvé les stratégies de l’établissement, les enseignants l’ajoutent aux cours via les points de placement standard de Blackboard. Les étapes exactes diffèrent entre Ultra Course View et Original Course View, donc les deux sont couvertes ci-dessous.

#### Ultra Course View

Ultra Course View est la valeur par défaut dans Blackboard Learn SaaS à partir de 2026.

1. Ouvrez le cours et allez à la page **Course Content**.
2. Survolez ou appuyez à l’endroit où vous voulez placer le fil de commentaires dans le plan et cliquez sur le bouton violet **+** (Add content).
3. Choisissez **Content Market**. Le panneau Content Market liste tous les outils LTI approuvés et les placements Building Block pour votre établissement.
4. Trouvez la vignette **FastComments** et cliquez dessus. Blackboard crée un élément de contenu à la position où vous avez ouvert le menu **+**.
5. Par défaut, l’élément apparaît dans le plan comme une entrée « Visible to students » pour les enseignants dont le réglage personnel **Hide from students** est désactivé. Si votre valeur par défaut est **Hidden**, l’élément est créé en mode masqué et vous activez le sélecteur de visibilité sur la ligne de l’élément lorsque vous êtes prêt.
6. Pour renommer l’élément, cliquez sur le titre dans le plan et tapez une nouvelle étiquette. Le titre que les étudiants voient dans le plan est indépendant de l’identifiant de fil FastComments, donc renommer est sans risque à tout moment.

Si vous ne voyez pas **Content Market** comme option, votre établissement a caché le placement. Vous pouvez aussi accéder au même sélecteur via **More tools** dans le même menu **+** sous le groupe **LTI Tools**.

#### Original Course View

Original Course View est toujours pris en charge dans Learn SaaS et reste l’expérience principale pour les sites Learn 9.1 auto-hébergés sur la ligne de correctifs CU de Q4 2024.

1. Ouvrez le cours et entrez dans une **Content Area** (par exemple, la zone **Information** ou **Content** par défaut dans le menu du cours).
2. Activez **Edit Mode** avec le commutateur en haut à droite de la page.
3. Cliquez sur **Build Content** dans la barre d’actions.
4. Sous le sous-menu **Learning Tools**, cliquez sur **FastComments**. Le sous-menu Learning Tools est rempli à partir des placements d’outils LTI 1.3 après qu’un administrateur a enregistré l’outil. Si vous ne le voyez pas, voyez la section des pièges ci-dessous.
5. Sur le formulaire **Create FastComments**, définissez :
   - **Name** : l’étiquette que les étudiants voient dans la zone de contenu.
   - **Description** : texte optionnel affiché au-dessus du fil intégré.
   - **Permit Users to View this Content** : bascule de disponibilité Oui/Non.
   - **Track Number of Views** : activez si vous voulez les statistiques de vues par élément de Blackboard. FastComments exécute ses propres analyses de façon indépendante.
   - **Date and Time Restrictions** : fenêtres optionnelles **Display After** / **Display Until**.
6. Soumettez. L’outil apparaît comme un élément cliquable dans la zone de contenu.

#### Intégration à l’intérieur d’un élément ou d’un document

Dans les deux vues de cours, les enseignants intègrent FastComments en ligne dans le corps d’un Item, d’un Document ou de tout champ riche via le bouton LTI Advantage de l’éditeur de contenu.

Ultra Course View :

1. Créez ou modifiez un **Document**.
2. Cliquez sur **Add content** à l’intérieur du corps du document à l’endroit où vous voulez que le fil apparaisse.
3. Dans la barre d’outils de l’éditeur, ouvrez le menu **Insert content** et cliquez sur **Content Market** (le point d’entrée LTI Advantage / Deep Linking).
4. Choisissez **FastComments**. FastComments renvoie une charge utile de deep-link et Blackboard insère un bloc intégré dans le corps du document à la position du curseur.
5. Enregistrez le document. Les étudiants voient le fil rendu en ligne lorsqu’ils le font défiler.

Original Course View :

1. Modifiez n’importe quel élément avec un corps en texte riche.
2. Dans la barre d’outils de l’éditeur de contenu, cliquez sur l’icône plus **Add Content** et choisissez **Content Market** (étiqueté **Add Content from External Tool** dans les anciens CU de Q4 2024).
3. Choisissez **FastComments**. L’éditeur insère un bloc espace réservé référant la ressource deep-linkée.
4. Soumettez l’élément.

Chaque intégration deep-link produit son propre fil FastComments, donc un Item avec deux blocs FastComments intégrés aura deux fils de commentaires indépendants.

#### Visibilité, conditions de diffusion et restrictions de groupe

Les éléments de contenu FastComments se comportent comme tout autre élément de contenu Blackboard pour les règles de contrôle d’accès qui leur sont appliquées.

- Ultra : cliquez sur le sélecteur de visibilité sur la ligne (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilité conditionnelle prend en charge les fenêtres de date/heure, les règles de performance liées aux éléments du carnet de notes, et les règles de membres liées aux groupes du cours.
- Original : ouvrez le menu contextuel de l’élément et choisissez **Adaptive Release** ou **Adaptive Release: Advanced** pour restreindre l’accès à l’outil par date, appartenance, note ou état de révision. Utilisez **Set Group Availability** sur l’élément pour restreindre à des groupes de cours spécifiques.

FastComments respecte la décision de contrôle d’accès de Blackboard. Si Blackboard cache l’élément à un étudiant, le lancement LTI n’a jamais lieu pour cet étudiant et il n’apparaît pas dans la vue modérateur.

#### Comportement dans le carnet de notes

FastComments ne renvoie pas de notes via LTI Advantage Assignment and Grade Services. Aucune colonne de notes n’est créée automatiquement pour les éléments de contenu FastComments.

Si votre locataire Blackboard est configuré pour créer automatiquement une colonne du carnet de notes pour chaque nouvel élément de contenu indépendamment des métadonnées de notation, une colonne vide apparaît quand même. Pour la masquer :

- Ultra : ouvrez le **Gradebook**, cliquez sur l’en-tête de colonne, choisissez **Edit**, et désactivez **Show to students** ainsi que **Include in calculations**. Ou utilisez **Delete** si votre établissement autorise la suppression de colonnes pour les éléments non notés.
- Original : ouvrez le **Grade Center**, cliquez sur le chevron de la colonne, choisissez **Hide from Users (on/off)**, et éventuellement **Hide from Instructor View** sous **Column Organization**.

#### Ce que voient les étudiants

Quand un étudiant ouvre l’élément FastComments ou fait défiler jusqu’à un bloc intégré :

1. Blackboard lance le message LTI 1.3 vers FastComments. L’étudiant est connecté via SSO en utilisant son identité Blackboard (nom, courriel, avatar, rôle) sans voir de formulaire de connexion.
2. Le fil de commentaires se rend dans l’iframe. Le fil, les réponses, les mentions et les réactions sont tous disponibles selon les paramètres du widget de commentaires configurés dans FastComments.
3. Leurs commentaires sont attribués à leur compte Blackboard. Si l’étudiant modifie ensuite son nom ou sa photo dans Blackboard, le prochain lancement mettra à jour le profil FastComments.

Correspondance des rôles de Blackboard vers FastComments :

- **System Administrator** et **Course Builder** correspondent à FastComments **admin**.
- **Instructor** et **Teaching Assistant** correspondent à FastComments **moderator**.
- **Student**, **Guest**, et **Observer** correspondent à FastComments **commenter**.

Les modérateurs voient les contrôles de modération (épingler, masquer, bannir, supprimer) en ligne sur chaque commentaire du fil.

#### Portée des fils

FastComments scope chaque fil par **(Blackboard host, course ID, resource link ID)**. Deux éléments FastComments dans le même cours produisent deux fils. Le même élément copié dans deux cours distincts (par exemple via la copie de cours) produit deux fils, parce que Blackboard émet un nouvel resource link ID lors de la copie. Pour conserver un fil partagé à travers les copies de cours, utilisez Deep Linking avec un URN de fil explicite configuré dans FastComments avant d’exécuter la copie.

#### Pièges spécifiques à Blackboard

**La vignette FastComments est absente du menu Build Content (Original) ou du Content Market (Ultra).** L’administrateur a approuvé l’outil mais laissé une stratégie d’établissement bloquant le placement pertinent. Allez dans **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifiez l’entrée FastComments, et confirmez que les placements **Course Content Tool** (Original) et **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) sont activés. Enregistrez et actualisez la page du cours.

**Erreur « Tool not configured for this context » ou « Tool is not deployed » au lancement.** La portée de déploiement enregistrée lors de l’enregistrement dynamique ne correspond pas au contexte institutionnel auquel appartient le cours. Dans l’entrée de fournisseur d’outils de Blackboard, vérifiez que le **Deployment ID** correspond à ce que FastComments affiche sur sa page de configuration LTI 1.3 pour ce locataire. S’ils diffèrent, supprimez le placement et relancez l’enregistrement dynamique depuis une URL d’enregistrement fraîche (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-le ici</a>).

**La hauteur de l’iframe semble fixe ou le contenu est tronqué.** Certains locataires Blackboard sont fournis avec une politique CSP stricte qui bloque le postMessage de redimensionnement d’iframe LTI par défaut. FastComments émet à la fois le message de style Canvas `lti.frameResize` et le message conforme à la spécification IMS `org.imsglobal.lti.frameResize` pour maximiser la compatibilité, mais une substitution CSP au niveau du locataire bloque le listener parent. Demandez à votre administrateur de confirmer que `*.fastcomments.com` figure sur la liste d’autorisation des outils LTI et qu’aucun en-tête CSP personnalisé ne filtre les événements postMessage. Le redimensionnement fonctionnera alors sans configuration supplémentaire.

**La copie de cours duplique les fils.** La copie de cours Blackboard génère de nouveaux resource link ID pour les placements LTI, donc les cours copiés commencent avec des fils vides. Cela est attendu. Si vous avez besoin que le cours copié hérite du fil original, configurez Deep Linking avec un URN de fil explicite avant la copie, ou contactez le support FastComments pour remapper les ID de fil en masse.

**L’étudiant voit une erreur générique Blackboard au lancement.** La cause est une revendication `email` manquante ou périmée. Confirmez que la stratégie d’établissement pour FastComments a **Role**, **Name**, et **Email Address** activés sous **User Fields to Send**. Enregistrez, puis relancez dans une nouvelle session du navigateur.