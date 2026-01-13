---
FastComments est une plateforme localisée. Tous nos widgets, courriels et notifications sont localisés.

Localisé signifie que nous affichons une langue et un format différents, selon l'emplacement de l'utilisateur et sa langue préférée. Nous déterminons cela à partir des informations fournies par le navigateur de l'utilisateur.

Nous pouvons personnaliser le texte du courriel en allant à l'onglet `Translations`, en sélectionnant un `Locale` et en modifiant le texte. Le texte modifié par rapport à la valeur par défaut est encadré dans l'interface utilisateur. Vous pouvez basculer entre les locales et enregistrer à la fin, sans perdre les modifications.

Le texte localisé est accessible via l'objet `TEXT`, par exemple : `<%= TEXT.INTRO %>`.

### Remarque SSO

Pour les intégrations SSO, si `locale` n'est pas spécifié, il sera mis à jour chaque fois que l'utilisateur accède au widget de commentaires avec une locale différente. Cela signifie que sa préférence de langue est automatiquement mise à jour et que les futurs courriels seront envoyés dans cette locale.

Cela peut aussi être défini manuellement en fournissant `locale` dans le payload SSO.

---