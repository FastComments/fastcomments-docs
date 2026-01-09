---
FastComments utilise des variables d'environnement pour la configuration. La liste suivante décrit toutes les variables prises en charge qui sont pertinentes pour On-Prem.

| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Type d'environnement.                                                                                                                              | Oui      | production, dev                                       |
| MONGO_URI                      |                             | URI de connexion à la BD.                                                                                                                          | Oui      |                                                       |
| MONGO_ENABLE_SSL               | false                       | Permet l'utilisation de SSL pour se connecter à la base de données.                                                                               | Non      | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Permet de valider le certificat par rapport à l'autorité de certification lors de la connexion à Mongo.                                            | Non      | true, false                                           |
| MONGO_SSL_CA                   |                             | Fichier pem de l'autorité de certification SSL de Mongo.                                                                                          | Non      | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Adresse courriel où les notifications importantes liées au système doivent être envoyées.                                                         | Non      | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Sel pour le hachage des adresses IP.                                                                                                              | Oui      |                                                       |
| SESSION_SECRET                 |                             | Clé utilisée pour signer les sessions.                                                                                                            | Oui      |                                                       |
| SESSION_STORE_SECRET           |                             | Clé utilisée pour signer/hacher les sessions en stockage. Doit être différente de SESSION_SECRET.                                                 | Oui      |                                                       |
| HOSTNAME                       |                             | Le nom d'hôte où FastComments est déployé (tableau de bord admin, etc.). NE DOIT PAS inclure le port ou le protocole.                              | Oui      | example.com                                           |
| HOST_ADDR                      |                             | Une URI accessible où FastComments est déployé (tableau de bord admin, etc.).                                                                     | Oui      | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Un chemin sur le système de fichiers local où se trouve la configuration des courriels (SMTP, mappages de domaines/fournisseurs, etc.).             | Oui      | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | En-tête "Nom de l'expéditeur" des courriels.                                                                                                      | Non      | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo du pied de page des courriels.                                                                                                               | Non      | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Remplace "defaultTransport" dans EMAIL_CONFIG_PATH. Utile pour déployer le même fichier de configuration sur différents environnements.            | Non      | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | L'ID de votre compte sur fastcomments.com. Utilisé pour enregistrer votre clé de licence.                                                         | Non      |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Une clé de licence sur site.                                                                                                                       | Non      |                                                       |
| GIPHY_API_KEY                  |                             | Clé API Giphy. Si non spécifiée, vous devriez créer une règle de configuration qui désactive le sélecteur de GIF.                                   | Non      |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Utilisé pour l'intégration Giphy. Peut aussi être remplacé par des règles de personnalisation du widget.                                          | Non      | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Utilisé pour les fonctionnalités propulsées par OpenAI, comme la détection anti-spam optionnelle basée sur GPT.                                   | Non      |                                                       |
| CDN_HOST_ADDR                  |                             | Le nom d'hôte d'où les ressources seront récupérées. Par défaut, la valeur de HOSTNAME.                                                            | Non      | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | Le nom d'hôte d'où sont récupérés les gros fichiers (comme les exportations). Par défaut, la valeur de CDN_HOST_ADDR.                               | Non      | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Où les gros fichiers, comme les exportations, doivent être stockés.                                                                               | Non      | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | Le nom d'hôte d'où les courriels doivent être envoyés.                                                                                             | Non      | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | Le nom du cookie fastcomments.                                                                                                                     | Non      |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | La valeur du champ "hostname" du cookie. Il est recommandé de préfixer avec un point.                                                             | Non      | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Utilisé pour les téléversements de fichiers utilisateur, avatars, etc. Par défaut, système de fichiers local si non défini.                         | Non      |                                                       |
| S3_SECRET_KEY                  |                             | Utilisé pour les téléversements de fichiers utilisateur, avatars, etc.                                                                             | Non      |                                                       |
| S3_REGION                      |                             | Utilisé pour les téléversements de fichiers utilisateur, avatars, etc.                                                                             | Non      |                                                       |
| S3_BUCKET                      |                             | Utilisé pour les téléversements de fichiers utilisateur, avatars, etc.                                                                             | Non      |                                                       |
| S3_HOST                        |                             | Utilisé pour les téléversements de fichiers utilisateur, avatars, etc.                                                                             | Non      |                                                       |
| CACHE_DIR                      |                             | Emplacement pour stocker le cache hors ligne optionnel, pour quand la BD n'est pas disponible. Rafraîchi périodiquement avec les 100 meilleurs fils de commentaires. | Non      |                                                       |
| BACKUP_DIR                     |                             | Emplacement pour stocker les données lorsque la BD n'est pas disponible. Si un commentaire est soumis lorsque la BD n'est pas disponible, il est stocké ici et traité plus tard. | Non      |                                                       |

Notez que toutes les variables liées aux domaines utilisent le suffixe `_HOST` ou `_ADDR`. La différence est :

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

Le `EMAIL_CONFIG_PATH` doit contenir un chemin vers un fichier JSON avec le format d'exemple suivant :

[inline-code-attrs-start title = 'Configuration de courriel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "defaultDKIM": {
        "domainName": "mycompany.org",
        "keySelector": "2024",
        "privateKey": "-----BEGIN PRIVATE KEY-----\nABCDEFG\n-----END PRIVATE KEY-----"
    },
    "providerTransports": {
        "yahoo.com": "specialTransport"
    },
    "defaultTransport": "mailgun",
    "transports": {
        "mailgun": {
            "host": "smtp.mailgun.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@somewhere.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        },
        "specialTransport": {
            "host": "smtp.someplace.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@example.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        }
    }
}
[inline-code-end]

Dans l'exemple ci‑dessus, nous définissons un transport de courriel `SMTP` par défaut nommé `mailgun`. Nous définissons également un transport spécial que nous utilisons spécifiquement pour les courriels `@yahoo.com`. Dans certains scénarios, il peut être souhaitable d'utiliser un fournisseur ou une adresse IP d'envoi spécifique pour un domaine afin d'optimiser la délivrabilité. Ceci est optionnel.

### DocumentDB

Lors de la connexion à `DocumentDB`, vous devrez spécifier `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` pour être compatible avec les paramètres par défaut.

---