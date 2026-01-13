---
FastComments est une plateforme localisée. Tous nos widgets, e-mails et notifications sont localisés.

Localisé signifie que nous affichons une langue et un formatage différents en fonction de l'emplacement de l'utilisateur et de sa langue préférée. Nous déterminons cela à partir des informations fournies par le navigateur de l'utilisateur.

Nous pouvons personnaliser le texte dans l'e-mail en allant dans l'onglet `Translations`, en sélectionnant une `Locale` et en modifiant le texte. Le texte modifié par rapport à la valeur par défaut est mis en évidence dans l'interface utilisateur. Vous pouvez basculer entre les locales et enregistrer à la fin, sans perdre les modifications.

Le texte localisé est accessible via l'objet `TEXT`, par exemple : `<%= TEXT.INTRO %>`.

### Remarque SSO

Pour les intégrations SSO, si `locale` n'est pas spécifié, il sera mis à jour à chaque fois que l'utilisateur accède au widget de commentaires avec une locale différente. Cela signifie que sa préférence linguistique est mise à jour automatiquement, et que les futurs e-mails seront envoyés dans cette locale.

Cela peut également être défini manuellement en fournissant `locale` dans la charge utile SSO.

---