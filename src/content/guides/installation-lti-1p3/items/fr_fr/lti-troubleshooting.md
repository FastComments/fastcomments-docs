#### « Jeton d'enregistrement introuvable, expiré ou déjà utilisé »

Le jeton dans votre URL d'enregistrement (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">à obtenir ici</a>) est valable 30 minutes et ne peut être utilisé qu'une seule fois. Si votre LMS a mis plus de temps, ou si l'enregistrement a été relancé après avoir réussi, le jeton sera rejeté. Générez une nouvelle URL dans la page de configuration LTI 1.3 de FastComments et recommencez.

#### « La plateforme a rejeté l'enregistrement »

Votre LMS a refusé la poignée de main d'enregistrement. Les causes les plus fréquentes :

- **L'outil est déjà enregistré avec le même nom de client.** Certaines plateformes (notamment D2L) rejettent un second enregistrement de « FastComments » tant que le précédent n'a pas été supprimé. Supprimez l'ancien outil dans votre LMS, puis réessayez.
- **Mauvais champ dans le LMS.** Assurez-vous d'avoir collé l'URL dans le champ **registration / tool initiation registration endpoint**, et non dans le champ de l'URL de lancement ou de connexion.
- **Le LMS ne prend pas réellement en charge l'enregistrement dynamique.** Les anciennes versions de Moodle et Blackboard annoncent LTI 1.3 mais n'autorisent que la configuration manuelle. Consultez la documentation de votre plateforme.

#### « Échec de récupération de la configuration de la plateforme »

FastComments n'a pas pu lire le document openid-configuration de votre LMS. C'est rare et cela signifie généralement que le LMS a fourni une URL de découverte mal formée ou inaccessible. Contactez le support de votre LMS.

#### Le lancement affiche « Configuration not found »

Soit la configuration dans FastComments a été supprimée, soit le lancement provient d'une paire `iss`/`client_id` que nous ne reconnaissons pas. Si vous avez supprimé puis réenregistré l'outil, demandez à votre LMS de supprimer puis de rajouter l'outil FastComments afin qu'il obtienne le nouveau client_id.

#### Le lancement affiche « Deployment not registered »

Vous avez lancé FastComments depuis un déploiement Brightspace/Moodle/Blackboard différent de celui du premier lancement. FastComments fige le `deployment_id` au premier lancement comme contrôle de sécurité. Pour ajouter un nouveau déploiement sous le même client, contactez le support - nous ajouterons l'ID de déploiement à la configuration.

#### Le lancement affiche « Unsupported message_type »

Le LMS a envoyé un message LTI que FastComments ne gère pas (par exemple `LtiSubmissionReviewRequest`). FastComments prend en charge uniquement le lancement standard resource-link et les flux de deep-linking. Contactez-nous si vous avez besoin d'un type de message spécifique.

#### L'iframe ne se redimensionne pas

La plupart des LMS dimensionnent automatiquement les iframes LTI. Si ce n'est pas le cas du vôtre, vérifiez que les paramètres de lancement du LMS autorisent l'outil à envoyer des événements postMessage au frame parent. FastComments émet à la fois des messages de redimensionnement de style Canvas (`lti.frameResize`) et conformes à la spécification IMS (`org.imsglobal.lti.frameResize`).