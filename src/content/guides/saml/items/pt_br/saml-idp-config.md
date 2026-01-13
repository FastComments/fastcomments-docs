Após configurar SAML no FastComments, você precisa configurar o FastComments como Provedor de Serviço no seu provedor de identidade.

### Configuração geral do IdP

A maioria dos provedores de identidade exige as seguintes informações para adicionar o FastComments como uma aplicação SAML:

#### Informações obrigatórias do Provedor de Serviço

Esses valores são gerados automaticamente e exibidos na sua página de configuração SAML do FastComments:

**ID da Entidade SP / Audiência**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Isso identifica de forma única sua instância do FastComments

**URL do Assertion Consumer Service (ACS)**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Onde seu IdP envia as respostas SAML após a autenticação

**SP Metadata URL** *(se suportado pelo seu IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Fornece a configuração SAML completa em formato XML

**URL de Login SAML**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Link direto para iniciar a autenticação SAML

### Atributos SAML obrigatórios

Configure seu provedor de identidade para enviar esses atributos nas respostas SAML:

#### Atributos essenciais

**Endereço de email** *(Obrigatório)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Identificação única do usuário e notificações
- **Format**: Endereço de email válido

#### Atributos opcionais

**Primeiro nome**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Nome exibido do usuário

**Sobrenome**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Nome exibido do usuário

**Funções** *(Importante para controle de acesso)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Atribuição de funções e permissões no FastComments
- **Format**: array de strings de função ou valores separados por vírgula

### Configurações comuns de provedores de identidade

#### Microsoft Azure AD

1. **Adicionar aplicativo corporativo**
   - Pesquise por "FastComments" ou crie um aplicativo SAML personalizado
   - Use as informações do SP fornecidas pelo FastComments

2. **Configurar atributos**
   - Email: `user.mail` or `user.userprincipalname`
   - Primeiro nome: `user.givenname`
   - Sobrenome: `user.surname`
   - Funções: `user.assignedroles` or directory groups

#### Okta

1. **Criar aplicativo SAML**
   - Use "Create New App" e selecione SAML 2.0
   - Configure com as informações do SP do FastComments

2. **Declarações de atributo**
   - Email: `user.email`
   - Primeiro nome: `user.firstName`
   - Sobrenome: `user.lastName`
   - Funções: `user.groups` or custom attributes

#### Google Workspace

1. **Adicionar aplicativo SAML**
   - Go to Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configure com as informações do SP do FastComments

2. **Mapeamento de atributos**
   - Email: Primary email
   - Primeiro nome: First name
   - Sobrenome: Last name
   - Funções: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Adicionar Relying Party Trust**
   - Use the FastComments metadata URL or manual configuration
   - Configure as informações do SP conforme fornecidas

2. **Regras de Claim**
   - Email: Email Address claim
   - Nome: Name ID claim
   - Funções: Group membership or custom claims

### Flexibilidade nos nomes de atributos

O FastComments aceita informações de função a partir de múltiplos nomes de atributos para acomodar diferentes configurações de IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Essa flexibilidade garante compatibilidade com vários provedores de identidade sem exigir convenções específicas de nomenclatura de atributos.

### Testando sua configuração

Após configurar seu provedor de identidade:

1. Salvar a configuração do IdP
2. Testar com uma conta de usuário de teste dedicada
3. Verificar se os atributos estão sendo enviados corretamente
4. Verificar se as funções estão mapeadas corretamente
5. Garantir que o fluxo de autenticação seja concluído com sucesso

A maioria dos provedores de identidade oferece ferramentas de teste SAML para validar a configuração antes de implantar para usuários em produção.