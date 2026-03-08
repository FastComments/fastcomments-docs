Les rôles Canvas sont automatiquement mappés aux rôles FastComments lors du lancement LTI. Aucune configuration manuelle n'est nécessaire.

#### Correspondance des rôles

| Rôle Canvas | Rôle FastComments | Autorisations |
|---|---|---|
| **Administrator** | Admin | Accès complet au compte, gestion de tous les commentaires et des paramètres |
| **Instructor** | Moderator | Modifier et supprimer des commentaires, épingler des fils, gérer les discussions |
| **Learner** | Commenter | Publier des commentaires, répondre, voter et utiliser les mentions |

#### Comment cela fonctionne

Lorsqu'un utilisateur lance FastComments depuis Canvas, le protocole LTI 1.3 inclut son rôle Canvas. FastComments lit ce rôle et assigne automatiquement les autorisations appropriées.

Si un utilisateur possède plusieurs rôles (par exemple un Instructor qui est également Admin), le rôle avec le niveau de privilège le plus élevé est utilisé.

---