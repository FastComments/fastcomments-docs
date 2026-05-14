Une fois FastComments enregistré dans votre LMS, les enseignants l'ajoutent aux cours de la même manière qu'ils ajoutent tout autre outil externe LTI.

#### D2L Brightspace

Dans la zone de contenu d'un cours :

1. Cliquez sur **Add Existing Activities** > **External Learning Tools**.
2. Sélectionnez **FastComments** dans la liste.
3. L'outil apparaît comme un sujet dans la zone de contenu. Ouvrez-le une fois pour initialiser le fil de commentaires pour cette ressource.

#### Moodle

Dans un cours :

1. Activez **Edit mode**.
2. Dans la section où vous souhaitez des commentaires, cliquez sur **Add an activity or resource**.
3. Choisissez **FastComments** dans le sélecteur d'activités.
4. Enregistrez. Les étudiants voient le fil de commentaires intégré dans la section.

#### Blackboard Learn

Dans un cours :

1. Accédez à une Content Area.
2. Cliquez sur **Build Content** > **FastComments** (under "Learning Tools").
3. Configurez un nom et soumettez.

#### Sakai

Les responsables du site ajoutent l'outil via **Site Info** > **Manage Tools** > faites défiler jusqu'à **External Tools** > sélectionnez **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments crée un fil de commentaires séparé par **(LMS instance, course, resource link)**. Cela signifie :

- Deux cours différents dans le même LMS obtiennent des fils séparés, même s'ils utilisent le même nom d'outil.
- Le même outil FastComments utilisé à deux endroits au sein d'un même cours crée deux fils.
- Deux installations Brightspace différentes sous le même compte FastComments obtiennent des fils distincts - le LMS hostname fait partie de l'identifiant du fil.

#### SSO

Les étudiants ne voient pas d'écran de connexion. Le LMS envoie leur identité (nom, e-mail, avatar, rôle) à FastComments via le lancement LTI, et FastComments les authentifie automatiquement. Leurs commentaires sont attribués à leur compte LMS.

Les utilisateurs ayant les rôles LMS **Instructor** ou **Administrator** sont automatiquement associés aux rôles modérateur/administrateur de FastComments pour le fil.