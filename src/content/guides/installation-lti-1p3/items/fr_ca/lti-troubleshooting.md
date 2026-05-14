---
#### "Jeton d'inscription introuvable, expiré ou déjà utilisé"

Le jeton dans votre URL d'enregistrement est valide pendant 30 minutes et ne peut être utilisé qu'une seule fois. Si votre LMS a pris plus de temps que cela, ou si l'enregistrement a été réessayé après avoir réussi, le jeton sera rejeté. Générez une nouvelle URL sur la page de configuration LTI 1.3 de FastComments et recommencez.

#### "La plateforme a rejeté l'inscription"

Votre LMS a refusé la poignée de main d'enregistrement. Les causes les plus fréquentes :

- **Outil déjà enregistré avec le même nom de client.** Certaines plateformes (notamment D2L) refusent un second enregistrement de "FastComments" tant que l'ancien n'a pas été supprimé. Supprimez l'ancien outil dans votre LMS, puis réessayez.
- **Mauvais champ dans le LMS.** Assurez-vous d'avoir collé l'URL dans le champ **registration / tool initiation registration endpoint**, et non dans le champ launch URL ou login URL.
- **Le LMS ne prend pas réellement en charge l'enregistrement dynamique.** Les anciennes versions de Moodle et Blackboard annoncent LTI 1.3 mais n'autorisent que la configuration manuelle. Consultez la documentation de votre plateforme.

#### "Impossible de récupérer la configuration de la plateforme"

FastComments n'a pas pu lire le document openid-configuration de votre LMS. C'est rare et cela signifie généralement que le LMS a fourni une URL de découverte malformée ou inaccessible. Contactez le support de votre LMS.

#### Le lancement affiche "Configuration introuvable"

Soit la configuration dans FastComments a été supprimée, soit le lancement provient d'une paire `iss`/`client_id` que nous ne reconnaissons pas. Si vous avez supprimé puis réenregistré, demandez à votre LMS de supprimer puis de réajouter l'outil FastComments afin qu'il obtienne le nouveau client_id.

#### Le lancement affiche "Déploiement non enregistré"

Vous avez lancé FastComments depuis un déploiement Brightspace/Moodle/Blackboard différent de celui dans lequel il a été lancé initialement. FastComments épingle le `deployment_id` au premier lancement comme vérification de sécurité. Pour ajouter un nouveau déploiement sous le même client, contactez le support — nous ajouterons l'ID de déploiement à la configuration.

#### Le lancement affiche "message_type non pris en charge"

Le LMS a envoyé un message LTI que FastComments ne gère pas (par ex. `LtiSubmissionReviewRequest`). FastComments prend en charge uniquement les flux standard de lancement resource-link et de deep-linking. Contactez-nous si vous avez besoin qu'un type de message spécifique soit ajouté.

#### L'iframe ne se redimensionne pas

La plupart des LMS redimensionnent automatiquement les iframes LTI. Si le vôtre ne le fait pas, vérifiez que les paramètres de lancement du LMS permettent à l'outil d'envoyer des événements postMessage au cadre parent. FastComments émet des messages de redimensionnement de type Canvas (`lti.frameResize`) et de la spécification IMS (`org.imsglobal.lti.frameResize`).

---