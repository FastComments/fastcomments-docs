Une fois FastComments enregistré dans votre LMS, les instructeurs l'ajoutent aux cours de la même manière qu'ils ajoutent n'importe quel autre outil externe LTI.

#### D2L Brightspace

Dans une zone de contenu du cours :

1. Cliquez sur **Ajouter des activités existantes** > **Outils d'apprentissage externes**.
2. Sélectionnez **FastComments** dans la liste.
3. L'outil apparaît comme un élément dans la zone de contenu. Ouvrez-le une fois pour initialiser le fil de commentaires pour cette ressource.

#### Moodle

Dans un cours :

1. Activez le **Mode d'édition**.
2. Dans la section où vous voulez des commentaires, cliquez sur **Ajouter une activité ou une ressource**.
3. Choisissez **FastComments** dans le sélecteur d'activités.
4. Enregistrez. Les étudiants voient le fil de commentaires intégré dans la section.

#### Blackboard Learn

Dans un cours :

1. Accédez à une zone de contenu.
2. Cliquez sur **Créer du contenu** > **FastComments** (sous « Outils d'apprentissage »).
3. Donnez un nom et soumettez.

#### Sakai

Les gestionnaires du site ajoutent l'outil via **Infos du site** > **Gérer les outils** > faites défiler jusqu'à **Outils externes** > sélectionnez **FastComments** > **Continuer**.

#### How Threads Are Scoped

FastComments crée un fil de commentaires séparé par **(LMS instance, course, resource link)**. Cela signifie :

- Deux cours différents dans le même LMS ont des fils séparés, même s'ils utilisent le même nom d'outil.
- Le même outil FastComments utilisé à deux endroits dans un même cours crée deux fils.
- Deux installations Brightspace différentes sous le même compte FastComments obtiennent des fils distincts - le nom d'hôte du LMS fait partie de l'identifiant du fil.

#### SSO

Les étudiants ne voient pas d'écran de connexion. Le LMS envoie leur identité (nom, courriel, avatar, rôle) à FastComments via le lancement LTI, et FastComments les connecte automatiquement. Leurs commentaires sont attribués à leur compte LMS.

Les utilisateurs ayant les rôles LMS **Instructor** ou **Administrator** sont automatiquement associés aux rôles modérateur/administrateur de FastComments pour le fil.