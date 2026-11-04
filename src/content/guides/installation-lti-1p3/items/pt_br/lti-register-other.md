#### Sakai

O Sakai suporta LTI 1.3 Dynamic Registration em versões com LTI Advantage. No Administration Workspace:

1. Faça login como administrador do Sakai e abra o **Administration Workspace**.
2. Escolha **External Tools** > **Install LTI 1.3 Tool**.
3. Cole a URL de registro do FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha aqui</a>) e envie.
4. Aprove a ferramenta quando o handshake for concluído.

A ferramenta então aparece em **External Tools** e pode ser adicionada aos sites pelos seus mantenedores.

#### Schoology

Instâncias Schoology Enterprise suportam LTI 1.3, mas a disponibilidade do Dynamic Registration varia conforme a implantação. Verifique com o gerente da sua conta Schoology.

Se o Dynamic Registration não estiver disponível na sua instância do Schoology, será necessário configurar a integração manualmente usando estes endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Depois que o Schoology fornecer um Client ID e um Deployment ID, entre em contato com o suporte do FastComments para registrar a configuração no seu tenant.

#### Outras Plataformas LTI 1.3

Qualquer LMS que siga a especificação IMS LTI 1.3 Advantage deve funcionar com a mesma URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha aqui</a>). Procure uma configuração rotulada como "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ou similar.

Se a sua plataforma suportar apenas a configuração manual do LTI 1.3, use os quatro endpoints listados na seção do Schoology acima e entre em contato com o suporte para finalizar.