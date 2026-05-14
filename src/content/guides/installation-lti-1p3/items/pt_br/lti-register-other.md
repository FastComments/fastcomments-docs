#### Sakai

O Sakai oferece suporte ao Registro Dinâmico LTI 1.3 em versões com LTI Advantage. Na Área de Trabalho de Administração:

1. Faça login como administrador do Sakai e abra a **Área de Trabalho de Administração**.
2. Escolha **Ferramentas Externas** > **Instalar Ferramenta LTI 1.3**.
3. Cole a URL de registro do FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>) e envie.
4. Aprove a ferramenta quando o handshake for concluído.

A ferramenta então aparece em **Ferramentas Externas** e pode ser adicionada a sites pelos seus mantenedores.

#### Schoology

Instâncias Enterprise do Schoology suportam LTI 1.3, mas a disponibilidade do Registro Dinâmico varia conforme a implantação. Verifique com seu gerente de conta do Schoology.

Se o Registro Dinâmico não estiver disponível na sua instância do Schoology, você precisará configurar a integração manualmente usando estes endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Depois que o Schoology fornecer um Client ID e um Deployment ID, entre em contato com o suporte do FastComments para registrar a configuração no seu tenant.

#### Other LTI 1.3 Platforms

Qualquer LMS que siga a especificação IMS LTI 1.3 Advantage deve funcionar com a mesma URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>). Procure por uma configuração rotulada como "Registro Dinâmico", "URL de registro da ferramenta", "endpoint de registro de iniciação da ferramenta" ou similar.

Se sua plataforma suportar apenas a configuração manual do LTI 1.3, use os quatro endpoints listados na seção do Schoology acima e entre em contato com o suporte para finalizar.