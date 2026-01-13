Configurar a autenticação SAML no FastComments requer tanto a configuração no painel administrativo quanto a configuração no seu provedor de identidade.

### Pré-requisitos

Antes de configurar o SAML, verifique se você tem:

- Um plano FastComments Flex ou Pro (SAML não está disponível no plano Creators)
- Acesso administrativo à sua conta FastComments
- Acesso administrativo ao seu provedor de identidade
- Os metadados SAML ou informações de certificado do seu IdP

### Acessando a Configuração SAML

1. Faça login no seu [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Navegue até **Configurações de API/SSO** na barra lateral esquerda
3. Clique no botão **Configuração SAML**

Se você não vir o botão Configuração SAML, verifique se:
- Sua conta possui o pacote necessário (Flex ou Pro)
- Você tem permissões administrativas
- Seu usuário possui papéis de API Admin ou Admin Admin

### Configuração Básica do SAML

#### Ativar Autenticação SAML

1. Marque a caixa **Ativar Autenticação SAML**
2. Isso ativa o SAML para seu tenant e torna os campos de configuração disponíveis

#### Campos Obrigatórios

**IdP Single Sign-On URL** *(Obrigatório)*
- A URL para onde os usuários serão redirecionados para autenticação
- Geralmente fornecida pelo seu provedor de identidade
- Exemplo: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Obrigatório)*
- O certificado público do seu provedor de identidade
- Usado para verificar a autenticidade das respostas SAML
- Deve incluir o certificado completo com os marcadores BEGIN/END
- Exemplo de formato:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Campos Opcionais

**IdP Entity ID / Issuer**
- Identifica seu provedor de identidade
- Se deixado em branco, padrão para sua URL do FastComments
- Deve corresponder ao issuer configurado no seu IdP

### Configuração Avançada

#### Configurações de Segurança

**Signature Algorithm**
- Padrão para SHA-256 (recomendado)
- Opções: SHA-1, SHA-256, SHA-512
- Deve corresponder à configuração do seu IdP

**Digest Algorithm**
- Padrão para SHA-256 (recomendado)
- Usado para o cálculo de digest nas respostas SAML
- Deve corresponder à configuração do seu IdP

**Name ID Format**
- Padrão para o formato Endereço de Email
- Determina como os identificadores de usuário são formatados
- Opções comuns: Endereço de Email, Persistente, Transitório

#### Criptografia (Opcional)

**Private Key for Decryption**
- Necessário apenas se seu IdP criptografar as assertions SAML
- Cole sua chave privada usada para descriptografar
- A maioria das implantações não requer criptografia das assertions

### Salvando a Configuração

1. Revise todas as configurações para precisão
2. Clique em **Salvar Configuração SAML**
3. O sistema validará sua configuração
4. Se for bem-sucedido, você verá uma mensagem de confirmação

### Próximos Passos

Após salvar sua configuração SAML do FastComments:

1. Configure seu provedor de identidade usando as informações do Provedor de Serviço
2. Teste o fluxo de autenticação
3. Configure papéis e permissões de usuário conforme necessário

As informações do Provedor de Serviço necessárias para a configuração do seu IdP serão exibidas assim que o SAML estiver ativado.