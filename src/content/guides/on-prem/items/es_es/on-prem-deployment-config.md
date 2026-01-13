FastComments utiliza variables de entorno para la configuración. La siguiente lista describe todas las variables compatibles que son relevantes para On-Prem.


| Variable                       | Default                     | Info                                                                                                                                               | Requerido | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | Tipo de entorno.                                                                                                                                    | Sí       | production, dev                                       |
| MONGO_URI                      |                             | URI de conexión a la base de datos.                                                                                                                | Sí       |                                                       |
| MONGO_ENABLE_SSL               | false                       | Permite usar SSL para conectarse a la base de datos.                                                                                              | No       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Habilita la validación del certificado contra la CA al conectarse a Mongo.                                                                        | No       | true, false                                           |
| MONGO_SSL_CA                   |                             | Archivo PEM de la CA SSL de Mongo.                                                                                                                 | No       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | Correo donde deben enviarse notificaciones importantes relacionadas con el sistema.                                                                | No       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | Salt para hashear direcciones IP.                                                                                                                  | Sí       |                                                       |
| SESSION_SECRET                 |                             | La clave usada para firmar las sesiones.                                                                                                           | Sí       |                                                       |
| SESSION_STORE_SECRET           |                             | La clave usada para firmar/hashear sesiones en el almacenamiento. Debe ser diferente a SESSION_SECRET.                                            | Sí       |                                                       |
| HOSTNAME                       |                             | El nombre de host donde se despliega FastComments (panel de administración, etc.). NO debe incluir puerto ni protocolo.                            | Sí       | example.com                                           |
| HOST_ADDR                      |                             | Una URI accesible donde se despliega FastComments (panel de administración, etc.).                                                                 | Sí       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | Una ruta en el sistema de archivos local donde se encuentra la configuración de correo (SMTP, mapeos de dominio/proveedor, etc.).                    | Sí       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Encabezado 'From Name' del correo electrónico.                                                                                                     | No       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo del pie de página del correo.                                                                                                                 | No       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | Anulación de 'defaultTransport' en EMAIL_CONFIG_PATH. Útil para desplegar el mismo archivo de configuración en diferentes entornos.                 | No       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | El ID de su cuenta en fastcomments.com. Se usa para registrar su clave de licencia.                                                               | No       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | Una clave de licencia on-prem.                                                                                                                      | No       |                                                       |
| GIPHY_API_KEY                  |                             | Clave API de Giphy. Si no se especifica, debe crear una regla de configuración que deshabilite el selector de GIFs.                                | No       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | Usado para la integración con Giphy. También puede ser anulado con reglas de personalización del widget.                                           | No       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | Usado para funciones con OpenAI como la detección de spam opcional basada en GPT.                                                                 | No       |                                                       |
| CDN_HOST_ADDR                  |                             | El nombre de host desde el que se obtendrán los assets. Por defecto toma el valor de HOSTNAME.                                                     | No       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | El nombre de host desde el que se obtendrán archivos grandes (como exportaciones). Por defecto toma el valor de CDN_HOST_ADDR.                       | No       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Dónde deben almacenarse archivos grandes, como exportaciones.                                                                                      | No       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | El nombre de host desde el que se deben enviar los correos.                                                                                        | No       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | El nombre de la cookie de fastcomments.                                                                                                            | No       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | El valor del campo 'hostname' de la cookie. Se recomienda prefijarlo con un punto.                                                                 | No       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | Usado para cargas de archivos de usuario, avatares, etc. Por defecto usa el sistema de archivos local si no está definido.                         | No       |                                                       |
| S3_SECRET_KEY                  |                             | Usado para cargas de archivos de usuario, avatares, etc.                                                                                          | No       |                                                       |
| S3_REGION                      |                             | Usado para cargas de archivos de usuario, avatares, etc.                                                                                          | No       |                                                       |
| S3_BUCKET                      |                             | Usado para cargas de archivos de usuario, avatares, etc.                                                                                          | No       |                                                       |
| S3_HOST                        |                             | Usado para cargas de archivos de usuario, avatares, etc.                                                                                          | No       |                                                       |
| CACHE_DIR                      |                             | Ubicación para almacenar la caché opcional fuera de línea, para cuando la BD no esté disponible. Se actualiza periódicamente con los 100 hilos principales. | No       |                                                       |
| BACKUP_DIR                     |                             | Ubicación para almacenar datos cuando la BD no está disponible. Si se envía un comentario cuando la BD no está disponible, va aquí y se procesa después. | No       |                                                       |

Tenga en cuenta que todas las variables relacionadas con dominios usan el sufijo `_HOST` o `_ADDR`. La diferencia es:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

El `EMAIL_CONFIG_PATH` debe contener una ruta a un archivo JSON con el siguiente formato de ejemplo:

[inline-code-attrs-start title = 'Configuración de correo'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

En el ejemplo anterior definimos un transporte de correo `SMTP` predeterminado llamado `mailgun`. También definimos un transporte especial que usamos específicamente para correos `@yahoo.com`. En algunos escenarios es deseable usar un proveedor específico o una IP de envío para un dominio para ajustar la entrega. Esto es opcional.

### DocumentDB

Al conectarse a `DocumentDB`, querrá especificar `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` para ser compatible con la configuración predeterminada.

---