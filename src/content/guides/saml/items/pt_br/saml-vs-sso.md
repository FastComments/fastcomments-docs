O FastComments oferece autenticação tanto por SSO quanto por SAML. Entender as diferenças ajuda você a escolher a abordagem certa para sua organização.

### Produções com SSO Simples/Seguro

O FastComments oferece dois fluxos de SSO diferentes para autenticação no widget de comentários através do seu site.
Isso é diferente do SAML e não requer SAML. Em vez disso, o SSO Simples simplesmente requer passar um objeto para
o widget de comentários, enquanto o SSO Seguro faz isso além de aplicar hash no payload com uma chave de API.

O SAML, por outro lado, autentica o usuário no produto inteiro (com base em suas permissões) *bem como* no widget de comentários
(se ele tiver cookies de terceiros habilitados para nosso domínio).

### Autenticação SAML

O SAML é um protocolo de autenticação de nível corporativo que fornece segurança mais robusta e capacidades de integração:

- **Implementação**: Requer configuração do Provedor de Identidade (IdP) e troca de certificados
- **Segurança**: Usa asserções XML assinadas e suporta criptografia
- **Caso de Uso**: Ideal para empresas com infraestrutura SAML existente (Active Directory, Okta, etc.)
- **Complexidade de Configuração**: Mais envolvida - requer configuração do IdP e gerenciamento de certificados
- **Recursos Corporativos**: Mapeamento avançado de funções, gerenciamento centralizado de usuários, trilhas de auditoria

### Quando Escolher SAML

Considere a autenticação SAML se sua organização:

- Já usa um provedor de identidade compatível com SAML (Okta, Azure AD, ADFS, etc.)
- Requer segurança e conformidade de nível corporativo
- Precisa de gerenciamento centralizado de usuários e controle de acesso
- Possui vários aplicativos usando SAML para autenticação
- Requer trilhas de auditoria detalhadas e relatórios de segurança

### Quando Escolher SSO Simples ou Seguro

Nossas soluções de SSO focadas no widget podem ser suficientes se você:

- Possui um sistema de autenticação personalizado
- Precisa de uma implementação rápida com configuração mínima
- Não requer integração com provedor de identidade corporativo
- Deseja controlar os dados do usuário diretamente do seu aplicativo
- Tem requisitos de segurança mais simples

O SSO Simples e o SSO Seguro são comumente usados para portais online, blogs, etc., onde o usuário já tem uma conta *através do seu site ou aplicativo*
mas não necessariamente usa SAML.