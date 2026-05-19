---
#### "Jeton d'enregistrement introuvable, expiré ou déjà utilisé"

Le token dans votre URL d'enregistrement (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-le ici</a>) est valide pendant 30 minutes et ne peut être utilisé qu'une seule fois. Si votre LMS a mis plus de temps, ou si l'enregistrement a été réessayé après avoir réussi, le token sera rejeté. Générez une nouvelle URL sur la page de configuration FastComments LTI 1.3 et recommencez.

#### "La plateforme a refusé l'enregistrement"

Votre LMS a refusé la négociation d'enregistrement. Les causes les plus courantes :

- **Outil déjà enregistré avec le même nom de client.** Certaines plateformes (notamment D2L) refusent une seconde inscription de "FastComments" tant que la précédente n'est pas supprimée. Supprimez l'ancien outil dans votre LMS, puis réessayez.
- **Mauvais champ dans le LMS.** Assurez-vous d'avoir collé l'URL dans le champ **registration / tool initiation registration endpoint**, et non dans le champ launch URL ou login URL.
- **Le LMS ne prend pas réellement en charge l'enregistrement dynamique.** Les anciennes versions de Moodle et Blackboard annoncent LTI 1.3 mais n'autorisent que la configuration manuelle. Consultez la documentation de votre plateforme.

#### "Échec de la récupération de la configuration de la plateforme"

FastComments n'a pas pu lire le document openid-configuration de votre LMS. C'est rare et signifie généralement que le LMS a fourni une URL de découverte mal formée ou inaccessible. Contactez le support de votre LMS.

#### Le lancement affiche "Configuration introuvable"

Soit la configuration dans FastComments a été supprimée, soit le lancement provenait d'une paire `iss`/`client_id` que nous ne reconnaissons pas. Si vous avez supprimé puis réenregistré, demandez à votre LMS de supprimer puis de réajouter l'outil FastComments afin qu'il obtienne le nouveau client_id.

#### Le lancement affiche "Déploiement non enregistré"

Vous avez lancé FastComments depuis un déploiement Brightspace/Moodle/Blackboard différent de celui dans lequel il a été lancé initialement. FastComments verrouille le `deployment_id` lors du premier lancement comme mesure de sécurité. Pour ajouter un nouveau déploiement sous le même client, contactez le support — nous ajouterons l'ID de déploiement à la configuration.

#### Le lancement affiche "Type de message non pris en charge"

Le LMS a envoyé un message LTI que FastComments ne gère pas (p. ex. `LtiSubmissionReviewRequest`). FastComments ne prend en charge que le lancement resource-link standard et les flux de deep-linking. Contactez-nous si vous avez besoin qu'un type de message spécifique soit ajouté.

#### L'iframe ne se redimensionne pas

La plupart des LMS redimensionnent automatiquement les iframes LTI. Si le vôtre ne le fait pas, vérifiez que les paramètres de lancement du LMS permettent à l'outil d'envoyer des événements postMessage à la fenêtre parente. FastComments émet des messages de redimensionnement de type Canvas (`lti.frameResize`) et selon la spécification IMS (`org.imsglobal.lti.frameResize`).

---