Les rôles Canvas sont automatiquement associés aux rôles FastComments lors du lancement LTI. Aucune configuration manuelle n'est nécessaire.

#### Correspondance des rôles

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Accès complet au compte, gestion de tous les commentaires et des paramètres |
| **Instructor** | Moderator | Modifier et supprimer des commentaires, épingler des fils de discussion, gérer les discussions |
| **Learner** | Commenter | Publier des commentaires, répondre, voter et utiliser les mentions |

#### Comment cela fonctionne

Lorsqu'un utilisateur lance FastComments depuis Canvas, le protocole LTI 1.3 inclut son rôle Canvas. FastComments lit ce rôle et attribue automatiquement les autorisations appropriées.

Si un utilisateur a plusieurs rôles (p. ex. un **Instructor** qui est aussi un **Admin**), le rôle avec le niveau de privilège le plus élevé est utilisé.

---