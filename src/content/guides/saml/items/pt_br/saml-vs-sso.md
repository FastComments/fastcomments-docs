O FastComments oferece autenticação SSO e SAML. Entender as diferenças ajuda você a escolher a abordagem certa para sua organização.

### Simple/Secure SSO Produções

O FastComments oferece dois fluxos SSO diferentes para autenticar no widget de comentários através do seu site.
Isto é diferente do SAML e não requer SAML. Em vez disso, o Simple SSO simplesmente exige passar um objeto para
o widget de comentários, enquanto o Secure SSO faz isso e ainda hash do payload com uma chave de API.

O SAML, por outro lado, autentica o usuário para todo o produto (com base nas permissões dele) *assim como* o widget de comentários
(se eles tiverem cookies de terceiros habilitados para o nosso domínio).

### Autenticação SAML

SAML é um protocolo de autenticação de nível corporativo que fornece capacidades de segurança e integração mais robustas:

- **Implementation**: Requer configuração do provedor de identidade (IdP) e troca de certificados
- **Security**: Usa assertions XML assinadas e suporta criptografia
- **Use Case**: Ideal para empresas com infraestrutura SAML existente (Active Directory, Okta, etc.)
- **Setup Complexity**: Mais envolvido - requer configuração do IdP e gerenciamento de certificados
- **Enterprise Features**: Mapeamento avançado de funções, gerenciamento centralizado de usuários, trilhas de auditoria

### When to Choose SAML

Considere a autenticação SAML se sua organização:

- Já usa um provedor de identidade compatível com SAML (Okta, Azure AD, ADFS, etc.)
- Requer segurança e conformidade em nível corporativo
- Precisa de gerenciamento centralizado de usuários e controle de acesso
- Possui múltiplas aplicações usando SAML para autenticação
- Requer trilhas de auditoria detalhadas e relatórios de segurança

### When to Choose Simple or Secure SSO

Nossas soluções de SSO focadas no widget podem ser suficientes se você:

- Possui um sistema de autenticação personalizado
- Precisa de implementação rápida com configuração mínima
- Não requer integração com provedores de identidade corporativos
- Quer controlar os dados dos usuários diretamente a partir da sua aplicação
- Tem requisitos de segurança mais simples

Simple e Secure SSO são comumente usados para portais online, blogs, etc., onde o usuário já tem uma conta *pelo seu site ou app*
mas não necessariamente usa SAML.