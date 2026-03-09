#### Ouvrir les clés de développeur dans Canvas

Connectez-vous à Canvas en tant qu'administrateur. Allez à **Admin** (dans la barre latérale gauche) > sélectionnez votre compte > **Developer Keys**.

#### Créer une clé de développeur LTI

Cliquez sur **+ Developer Key** et sélectionnez **LTI Key**.

Dans le formulaire de configuration :

1. Dans le champ **Redirect URIs** (côté gauche), collez l’**Launch URL** provenant de la page de configuration FastComments.
2. À droite, définissez **Method** sur **Enter URL**.
3. Collez l’**Configuration URL** que vous avez copiée depuis FastComments dans le champ **JSON URL**.
4. Canvas chargera automatiquement la configuration LTI.
5. Donnez un nom à la clé (p. ex. "FastComments").
6. Cliquez sur **Save**.

#### Activer la clé de développeur

Après l’enregistrement, la nouvelle clé apparaîtra dans le tableau Developer Keys avec son **State** réglé sur **OFF**. Cliquez sur le commutateur pour le définir sur **ON**. Canvas peut vous demander de confirmer — cliquez sur **Allow** pour activer la clé.

#### Copier le Client ID

Le tableau Developer Keys affiche un **Client ID** numérique dans la colonne Détails (p. ex. `17000000000042`). Copiez ce numéro — vous en aurez besoin à l’étape suivante.