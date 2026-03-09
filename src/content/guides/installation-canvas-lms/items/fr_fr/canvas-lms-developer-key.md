#### Ouvrir Developer Keys dans Canvas

Connectez-vous à Canvas en tant qu'administrateur. Allez dans **Admin** (dans la barre latérale gauche) > sélectionnez votre compte > **Developer Keys**.

#### Créer une LTI Developer Key

Cliquez sur **+ Developer Key** et sélectionnez **LTI Key**.

Dans le formulaire de configuration:

1. Dans le champ **Redirect URIs** (côté gauche), collez le **Launch URL** depuis la page de configuration FastComments.
2. À droite, définissez **Method** sur **Enter URL**.
3. Collez le **Configuration URL** que vous avez copié depuis FastComments dans le champ **JSON URL**.
4. Canvas chargera automatiquement la configuration LTI.
5. Donnez un nom à la clé (par ex. "FastComments").
6. Cliquez sur **Save**.

#### Activer la Developer Key

Après l'enregistrement, la nouvelle clé apparaîtra dans le tableau Developer Keys avec son **State** réglé sur **OFF**. Cliquez sur le bouton bascule pour le régler sur **ON**. Canvas peut vous demander de confirmer — cliquez sur **Allow** pour activer la clé.

#### Copier le Client ID

Le tableau Developer Keys affiche un **Client ID** numérique dans la colonne Details (par ex. `17000000000042`). Copiez ce numéro - vous en aurez besoin à l'étape suivante.