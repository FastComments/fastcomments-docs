Pour les développeurs que vous ne souhaitez pas rendre `Administrators`, envisagez de créer un utilisateur `Administrator`
avec les autorisations suivantes:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Cet ensemble d'autorisations donnera à un développeur tout ce dont il a besoin pour configurer FastComments ainsi
que la visibilité dans le système requise pour s'assurer qu'il fonctionne.

Les raisons de ces autorisations sont les suivantes:

1. **Analytics Admin**: Ceci peut être utilisé pour déboguer l'utilisation de FastComments.
2. **Customizations Admin**: Ceci sera requis pour appliquer un style personnalisé au widget de commentaires.
3. **Data Management Admin**: Ceci sera requis pour gérer les importations et exportations, et configurer les webhooks.
4. **Comment Moderation Admin**: Ceci sera requis pour voir les données des commentaires, au moins pendant la configuration.
5. **API/SSO Admin**: Ceci leur permettra de récupérer les API keys directement depuis notre plateforme. Nous considérons
cela plus sûr que si un `Administrator` les copie pour eux et envoie l'API Secret par courriel, ce qui peut ne pas être très sécurisé.