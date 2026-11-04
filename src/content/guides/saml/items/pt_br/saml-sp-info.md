Quando o SAML está habilitado no FastComments, o sistema gera automaticamente as informações do Service Provider (SP) que você precisa configurar no seu provedor de identidade.

### Acessando as Informações do Service Provider

As informações do SP são exibidas na sua página de configuração SAML após habilitar a autenticação SAML. Essas informações incluem todos os detalhes que seu provedor de identidade precisa para estabelecer a relação de confiança SAML.

### Endpoints do Service Provider

#### SP Entity ID / Audience
**Finalidade**: Identifica exclusivamente sua instância do FastComments como um service provider
**Formato**: `https://fastcomments.com/saml/{your-tenant-id}`
**Uso**: Configure isso como Entity ID ou Audience no seu IdP

Este identificador garante que as respostas SAML sejam destinadas ao seu tenant específico do FastComments e evita que respostas SAML sejam aceitas por outras instâncias.

#### Assertion Consumer Service (ACS) URL
**Finalidade**: O endpoint onde seu IdP envia as respostas SAML após a autenticação do usuário
**Formato**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Uso**: Configure isso como ACS URL ou Reply URL no seu IdP

É para onde os usuários são redirecionados após a autenticação bem-sucedida com seu provedor de identidade, junto com a asserção SAML contendo as informações do usuário.

#### SP Metadata URL
**Finalidade**: Fornece a configuração SAML completa em formato XML padrão
**Formato**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Uso**: Alguns IdPs podem importar a configuração automaticamente usando esta URL

A URL de metadados contém todas as informações necessárias do SP em formato XML, facilitando a configuração automática de provedores de identidade compatíveis.

#### SAML Login URL
**Finalidade**: Link direto para iniciar a autenticação SAML do seu tenant
**Formato**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Uso**: Direcione os usuários diretamente para a autenticação SAML ou teste o fluxo

Você pode usar esta URL para testar a autenticação SAML ou fornecer aos usuários um link direto para entrar via SAML.

### Suporte a Bindings SAML

O FastComments suporta os seguintes bindings SAML:

#### HTTP-POST Binding
- **Método Principal**: Binding mais comum para respostas SAML
- **Segurança**: A resposta SAML é enviada via HTTP POST para a ACS URL
- **Uso**: Recomendado para ambientes de produção

#### HTTP-Redirect Binding
- **Método Alternativo**: Resposta SAML enviada via redirecionamento HTTP
- **Limitações**: Tamanho de payload limitado devido a restrições de comprimento de URL
- **Uso**: Suportado, mas HTTP-POST é preferido

### Política de Name ID

O FastComments configura a seguinte política de Name ID nas requisições SAML:

- **Formato Padrão**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Formatos Alternativos**: Persistent, Transient, Unspecified (configurável)
- **Requisito**: O endereço de e-mail é usado como identificador principal do usuário

### Atributos das Requisições SAML

Ao iniciar a autenticação SAML, o FastComments envia requisições com estas características:

#### Assinatura de Requisições
- **Status**: Opcional (configurável)
- **Algoritmo**: Corresponde ao algoritmo de assinatura configurado
- **Certificado**: Usa o certificado específico do tenant se a assinatura de requisições estiver habilitada

#### Atributos Solicitados
O FastComments solicita os seguintes atributos nas SAML AuthnRequests:

- **Email**: Necessário para identificação do usuário
- **First Name**: Opcional, para fins de exibição  
- **Last Name**: Opcional, para fins de exibição
- **Roles/Groups**: Opcional, para controle de acesso e permissões

### Copiando as Informações do SP

A página de configuração SAML fornece campos clicáveis que copiam automaticamente as informações do SP para a sua área de transferência:

1. Clique em qualquer campo de informação do SP (Entity ID, ACS URL, etc.)
2. O valor é copiado automaticamente para a sua área de transferência
3. Cole o valor na configuração do seu provedor de identidade
4. Um breve destaque indica que a cópia foi bem-sucedida

Isso facilita a transferência precisa das informações do SP para o seu IdP sem erros de digitação.

### Informações do Certificado do SP

#### Uso do Certificado
- **Finalidade**: Criptografa as comunicações e verifica a identidade do SP
- **Rotação**: Os certificados são gerenciados automaticamente pelo FastComments
- **Acesso**: Os certificados públicos estão disponíveis via metadata URL

#### Detalhes do Certificado
- **Algoritmo**: RSA-2048 ou superior
- **Validade**: Os certificados são renovados automaticamente antes da expiração
- **Distribuição**: Disponíveis através dos metadados SAML padrão

### Solução de Problemas da Configuração do SP

Se o seu provedor de identidade relatar problemas com as informações do SP:

1. **Verifique as URLs**: Certifique-se de que todas as URLs usam HTTPS e incluem o tenant ID correto
2. **Verifique os Metadados**: Use a metadata URL para verificar a configuração
3. **Teste a Conectividade**: Certifique-se de que seu IdP consegue acessar os endpoints do FastComments
4. **Valide o Formato**: Confirme se seu IdP suporta o formato das informações do SP

Problemas comuns incluem:
- Tenant ID incorreto nas URLs
- Problemas de conectividade de rede entre o IdP e o FastComments
- IdP esperando formatos de URL diferentes ou opções de configuração adicionais