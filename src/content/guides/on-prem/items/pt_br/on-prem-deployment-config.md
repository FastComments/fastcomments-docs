FastComments utiliza variáveis de ambiente para configuração. A lista a seguir descreve todas as variáveis suportadas que são relevantes para On-Prem.


| Variável                       | Padrão                      | Informações                                                                                                                                        | Obrigatório | Exemplos ou Valores Válidos                         |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|------------:|-----------------------------------------------------|
| NODE_ENV                       |                             | Tipo de ambiente.                                                                                                                                  | Sim        | production, dev                                     |
| MONGO_URI                      |                             | URI de conexão com o BD.                                                                                                                           | Sim        |                                                     |
| MONGO_ENABLE_SSL               | false                       | Habilita o uso de SSL para conectar-se ao banco de dados.                                                                                         | Não        | true, false                                         |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | Habilita a validação do certificado contra a CA ao conectar-se ao Mongo.                                                                          | Não        | true, false                                         |
| MONGO_SSL_CA                   |                             | Arquivo PEM da CA para SSL do Mongo.                                                                                                              | Não        | /path/to/some-cert.pem                              |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | E-mail para onde devem ser enviadas notificações importantes relacionadas ao sistema.                                                             | Não        | admin-group@bigcorp.com                             |
| IP_HASH_SALT                   |                             | Salt para hashing de endereços IP.                                                                                                                | Sim        |                                                     |
| SESSION_SECRET                 |                             | Chave usada para assinar sessões.                                                                                                                 | Sim        |                                                     |
| SESSION_STORE_SECRET           |                             | Chave usada para assinar/hash das sessões no armazenamento. Deve ser diferente de SESSION_SECRET.                                                | Sim        |                                                     |
| HOSTNAME                       |                             | O hostname onde o FastComments está implantado (painel de administração etc). NÃO deve incluir porta ou protocolo.                                 | Sim        | example.com                                         |
| HOST_ADDR                      |                             | Uma URI acessível onde o FastComments está implantado (painel de administração etc).                                                              | Sim        | https://example.com                                 |
| EMAIL_CONFIG_PATH              |                             | Um caminho no sistema de arquivos local onde fica a configuração de e-mail (SMTP, mapeamentos de domínio/provedor, etc).                           | Sim        | /my/config.json                                     |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | Cabeçalho "From Name" do e-mail.                                                                                                                   | Não        | My Company Name                                     |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | Logo do rodapé do e-mail.                                                                                                                          | Não        | https://exmaple.com/footer.png                      |
| EMAIL_DEFAULT_TRANSPORT        |                             | Sobrescrita para "defaultTransport" em EMAIL_CONFIG_PATH. Útil para implantar o mesmo arquivo de configuração em ambientes diferentes.             | Não        | myTransportName                                     |
| ON_PREM_TENANT_ID              |                             | O ID da sua conta em fastcomments.com. Usado para registrar sua chave de licença.                                                                 | Não        |                                                     |
| ON_PREM_LICENSE_KEY            |                             | Uma chave de licença on-prem.                                                                                                                      | Não        |                                                     |
| GIPHY_API_KEY                  |                             | Chave de API do Giphy. Se não especificada, você deve criar uma regra de configuração que desative o seletor de GIFs.                              | Não        |                                                     |
| GIPHY_DEFAULT_RATING           | pg                          | Usada para integração com o Giphy. Também pode ser sobrescrita com regras de customização do widget.                                              | Não        | g, pg, pg-13, r                                     |
| OPENAI_SECRET_KEY              |                             | Usada para recursos com OpenAI, como detecção de spam opcional baseada em GPT.                                                                    | Não        |                                                     |
| CDN_HOST_ADDR                  |                             | O hostname de onde os assets serão buscados. Por padrão, usa o valor de HOSTNAME.                                                                 | Não        | example.com                                         |
| LARGE_FILE_HOST_ADDR           |                             | O hostname de onde arquivos grandes (como exports) são buscados. Por padrão, usa o valor de CDN_HOST_ADDR.                                        | Não        | example.com                                         |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | Onde arquivos grandes, como exports, devem ser armazenados.                                                                                       | Não        | local_disk, s3                                      |
| FROM_EMAIL_HOST                |                             | O hostname de onde os e-mails devem ser enviados.                                                                                                 | Não        | example.com                                         |
| COOKIE_ID                      | fastcomments.sid            | O nome do cookie do fastcomments.                                                                                                                  | Não        |                                                     |
| COOKIE_HOSTNAME                | .fastcomments.com           | O valor do campo "hostname" do cookie. Recomendado prefixar com ponto.                                                                            | Não        | .example.com                                        |
| S3_ACCESS_KEY                  |                             | Usado para uploads de arquivos de usuário, avatares, etc. Usa o sistema de arquivos local por padrão se não definido.                              | Não        |                                                     |
| S3_SECRET_KEY                  |                             | Usado para uploads de arquivos de usuário, avatares, etc.                                                                                         | Não        |                                                     |
| S3_REGION                      |                             | Usado para uploads de arquivos de usuário, avatares, etc.                                                                                         | Não        |                                                     |
| S3_BUCKET                      |                             | Usado para uploads de arquivos de usuário, avatares, etc.                                                                                         | Não        |                                                     |
| S3_HOST                        |                             | Usado para uploads de arquivos de usuário, avatares, etc.                                                                                         | Não        |                                                     |
| CACHE_DIR                      |                             | Local para armazenar cache offline opcional, para quando o BD não estiver disponível. Periodicamente atualizado com os 100 principais threads.      | Não        |                                                     |
| BACKUP_DIR                     |                             | Local para armazenar dados para quando o BD não estiver disponível. Se um comentário for enviado quando o BD não estiver disponível, vai para aqui e é processado depois. | Não        |                                                     |

Observe que todas as variáveis relacionadas a domínio usam o sufixo `_HOST` ou `_ADDR`. A diferença é:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

O `EMAIL_CONFIG_PATH` deve conter um caminho para um arquivo JSON com o seguinte formato de exemplo:

[inline-code-attrs-start title = 'Configuração de E-mail'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

No exemplo acima definimos um transporte de e-mail `SMTP` padrão chamado `mailgun`. Também definimos um transporte especial que usamos especificamente para e-mails `@yahoo.com`. Em alguns cenários é desejável usar um provedor específico ou um IP de envio para um domínio, para ajustar a entrega. Isso é opcional.

### DocumentDB

Ao conectar-se ao `DocumentDB` você deverá especificar `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` para ser compatível com as configurações padrão.

---