#### "Registration token not found, expired, or already used"

Le token dans votre registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) est valide pendant 30 minutes et ne peut être utilisé qu'une seule fois. Si votre LMS a mis plus de temps que cela, ou si l'enregistrement a été relancé après avoir réussi, le token sera rejeté. Générez une nouvelle URL sur la page de configuration FastComments LTI 1.3 et recommencez.

#### "Platform rejected registration"

Votre LMS a refusé la poignée de main d'enregistrement. Les causes les plus courantes :

- **Tool already registered with the same client name.** Certaines plateformes (notamment D2L) refusent un second enregistrement de "FastComments" tant que le précédent n'a pas été supprimé. Supprimez l'ancien outil dans votre LMS, puis réessayez.
- **Wrong field in the LMS.** Assurez-vous d'avoir collé l'URL dans le champ **registration / tool initiation registration endpoint**, et non dans le champ launch URL ou login URL.
- **The LMS doesn't actually support Dynamic Registration.** Les anciennes versions de Moodle et Blackboard annoncent LTI 1.3 mais n'autorisent que la configuration manuelle. Consultez la documentation de votre plateforme.

#### "Failed to fetch platform configuration"

FastComments n'a pas pu lire le document openid-configuration de votre LMS. C'est rare et signifie généralement que le LMS a fourni une URL de découverte malformée ou injoignable. Contactez le support de votre LMS.

#### Launch shows "Configuration not found"

Soit la configuration dans FastComments a été supprimée, soit le lancement provenait d'une paire `iss`/`client_id` que nous ne reconnaissons pas. Si vous avez supprimé puis réenregistré, demandez à votre LMS de supprimer puis de réajouter l'outil FastComments afin qu'il obtienne le nouveau client_id.

#### Launch shows "Deployment not registered"

Vous avez lancé FastComments depuis un déploiement Brightspace/Moodle/Blackboard différent de celui dans lequel il a été lancé initialement. FastComments associe le `deployment_id` lors du premier lancement comme mesure de sécurité. Pour ajouter un nouveau déploiement sous le même client, contactez le support — nous ajouterons l'ID de déploiement à la configuration.

#### Launch shows "Unsupported message_type"

Le LMS a envoyé un message LTI que FastComments ne gère pas (par ex. `LtiSubmissionReviewRequest`). FastComments prend en charge uniquement le lancement standard resource-link et les flux de deep-linking. Contactez-nous si vous avez besoin qu'un type de message spécifique soit ajouté.

#### Iframe doesn't resize

La plupart des LMS redimensionnent automatiquement les iframes LTI. Si le vôtre ne le fait pas, vérifiez que les paramètres de lancement de la plateforme autorisent l'outil à envoyer des événements postMessage au cadre parent. FastComments émet à la fois des messages de redimensionnement de type Canvas (`lti.frameResize`) et conformes à la spécification IMS (`org.imsglobal.lti.frameResize`).