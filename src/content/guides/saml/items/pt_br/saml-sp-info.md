Quando o SAML é habilitado no FastComments, o sistema gera automaticamente informações do Provedor de Serviço (SP) que você precisa configurar no seu provedor de identidade.

### Acessando as Informações do Provedor de Serviço

As informações do SP são exibidas na sua página de configuração SAML após habilitar a autenticação SAML. Essas informações incluem todos os detalhes que seu provedor de identidade precisa para estabelecer a relação de confiança SAML.

### Endpoints do Provedor de Serviço

#### SP Entity ID / Audience
**Purpose**: Identifica unicamente sua instância do FastComments como um provedor de serviço  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Usage**: Configure isto como o Entity ID ou Audience no seu IdP

Esse identificador assegura que as respostas SAML sejam destinadas ao seu tenant específico do FastComments e impede que respostas SAML sejam aceitas por outras instâncias.

#### Assertion Consumer Service (ACS) URL
**Purpose**: O endpoint onde seu IdP envia as respostas SAML após a autenticação do usuário  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Usage**: Configure isto como a ACS URL ou Reply URL no seu IdP

É aqui que os usuários são redirecionados após a autenticação bem-sucedida com seu provedor de identidade, junto com a asserção SAML contendo as informações do usuário.

#### SP Metadata URL
**Purpose**: Fornece a configuração SAML completa em formato XML padrão  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Usage**: Alguns IdPs podem importar automaticamente a configuração usando esta URL

A URL de metadata contém todas as informações necessárias do SP em formato XML, facilitando a configuração de provedores de identidade compatíveis automaticamente.

#### SAML Login URL
**Purpose**: Link direto para iniciar a autenticação SAML para seu tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Usage**: Direcione usuários diretamente para a autenticação SAML ou teste o fluxo

Você pode usar esta URL para testar a autenticação SAML ou fornecer aos usuários um link direto para entrar via SAML.

### Suporte a Binding SAML

O FastComments suporta os seguintes bindings SAML:

#### HTTP-POST Binding
- **Primary Method**: Método mais comum para respostas SAML  
- **Security**: A resposta SAML é enviada via HTTP POST para a ACS URL  
- **Usage**: Recomendado para implantações em produção

#### HTTP-Redirect Binding
- **Alternative Method**: Resposta SAML enviada via redirecionamento HTTP  
- **Limitations**: Tamanho de payload limitado devido às restrições de comprimento da URL  
- **Usage**: Suportado, mas HTTP-POST é preferido

### Política de Name ID

O FastComments configura a seguinte política de Name ID nas requisições SAML:

- **Default Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative Formats**: Persistent, Transient, Unspecified (configuráveis)  
- **Requirement**: O endereço de e-mail é usado como o identificador primário do usuário

### Atributos da Requisição SAML

Ao iniciar a autenticação SAML, o FastComments envia requisições com essas características:

#### Request Signing
- **Status**: Opcional (configurável)  
- **Algorithm**: Corresponde ao algoritmo de assinatura configurado  
- **Certificate**: Usa certificado específico do tenant se a assinatura da requisição estiver habilitada

#### Requested Attributes
O FastComments solicita os seguintes atributos nas AuthnRequests SAML:

- **Email**: Obrigatório para identificação do usuário  
- **First Name**: Opcional para fins de exibição  
- **Last Name**: Opcional para fins de exibição  
- **Roles/Groups**: Opcional para controle de acesso e permissões

### Copiando Informações do SP

A página de configuração SAML fornece campos clicáveis que copiam automaticamente as informações do SP para sua área de transferência:

1. Clique em qualquer campo de informação do SP (Entity ID, ACS URL, etc.)  
2. O valor é copiado automaticamente para sua área de transferência  
3. Cole o valor na configuração do seu provedor de identidade  
4. Um breve destaque indica cópia bem-sucedida

Isso facilita transferir com precisão as informações do SP para seu IdP sem erros de digitação.

### Informações do Certificado do SP

#### Uso do Certificado
- **Purpose**: Criptografa as comunicações e verifica a identidade do SP  
- **Rotation**: Certificados são gerenciados automaticamente pelo FastComments  
- **Access**: Certificados públicos estão disponíveis via URL de metadata

#### Detalhes do Certificado
- **Algorithm**: RSA-2048 ou superior  
- **Validity**: Certificados são renovados automaticamente antes da expiração  
- **Distribution**: Disponível através do metadata SAML padrão

### Solucionando problemas na configuração do SP

Se seu provedor de identidade reportar problemas com as informações do SP:

1. **Verify URLs**: Garanta que todas as URLs usem HTTPS e incluam o tenant ID correto  
2. **Check Metadata**: Use a URL de metadata para verificar a configuração  
3. **Test Connectivity**: Certifique-se de que seu IdP pode acessar os endpoints do FastComments  
4. **Validate Format**: Confirme que seu IdP suporta o formato das informações do SP

Problemas comuns incluem:
- Tenant ID incorreto nas URLs  
- Problemas de conectividade de rede entre o IdP e o FastComments  
- IdP esperando formatos de URL diferentes ou opções de configuração adicionais