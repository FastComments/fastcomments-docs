SAML (Security Assertion Markup Language) é um padrão aberto baseado em XML para a troca de dados de autenticação e autorização entre partes, 
particularmente entre um provedor de identidade (IdP) e um provedor de serviço (SP).

### Como o SAML funciona

O SAML possibilita o single sign-on (SSO) permitindo que os usuários se autentiquem uma vez com seu provedor de identidade e então acessem múltiplas aplicações 
sem reintroduzir credenciais. Quando um usuário tenta acessar o FastComments:

1. **Solicitação de Autenticação**: O FastComments redireciona o usuário para seu provedor de identidade
2. **Autenticação do Usuário**: O usuário se autentica no seu IdP (e.g., Active Directory, Okta, Azure AD)
3. **Resposta SAML**: O IdP envia uma asserção SAML assinada de volta ao FastComments
4. **Acesso do Usuário**: O FastComments valida a asserção e concede acesso ao usuário autenticado

### Benefícios do SAML

- **Segurança Aprimorada**: A autenticação centralizada reduz riscos de segurança relacionados a senhas
- **Melhora na Experiência do Usuário**: Os usuários fazem login uma vez e acessam múltiplas aplicações sem reintroduzir credenciais
- **Conformidade**: Ajuda a atender requisitos regulatórios para controle de acesso e trilhas de auditoria
- **Controle Administrativo**: Administradores de TI mantêm o gerenciamento centralizado de usuários

### Suporte ao SAML 2.0

O FastComments implementa o SAML 2.0, a versão do padrão SAML mais amplamente adotada. Nossa implementação suporta:

- Bindings HTTP-POST e HTTP-Redirect
- Respostas e asserções SAML assinadas
- Asserções criptografadas (opcional)
- Múltiplos algoritmos de assinatura e digest
- Diversos formatos de identificador de nome