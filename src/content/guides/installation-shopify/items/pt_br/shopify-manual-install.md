Se você não consegue instalar o [app da Shopify App Store](https://apps.shopify.com/fastcomments), ainda é possível adicionar o FastComments editando seu tema. Este caminho é útil quando você deseja conectar um tenant do FastComments que já possui, ou quando está incorporando em uma vitrine Shopify onde o app não é uma opção.

A instalação via app é o caminho suportado para a maioria das lojas. Use este método apenas se o app não for adequado.

### Passo 1: Desativar os comentários nativos da Shopify

No admin da Shopify, vá para **Blog posts > Manage blogs**, abra cada blog e defina **Comments are disabled** no painel à direita. Salve.

Isso impede que o sistema de comentários integrado da Shopify apareça junto com o FastComments.

### Passo 2: Abrir o template de blog do tema

No admin da Shopify:

1. Vá para **Online Store > Themes**.
2. Sob o seu tema atual, clique em **Actions > Edit code**.
3. No navegador de arquivos à esquerda, abra **Sections** e clique em `main-article.liquid`.

Este é o template que a Shopify renderiza para um artigo de blog individual.

### Passo 3: Colar o snippet do FastComments

Role até aproximadamente a linha 100 de `main-article.liquid`, logo após o `</div>` de fechamento do corpo do artigo. Cole o seguinte snippet:

[inline-code-attrs-start title = 'Snippet do FastComments para Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Substitua `"demo"` pelo seu próprio Tenant ID obtido em [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Clique em **Save**.

### Passo 4: Autorizar o domínio da sua loja

Abra uma postagem do blog na sua loja publicada. Se você vir um erro de autorização em vez do widget de comentários, o FastComments precisa saber que sua loja está autorizada a usar este tenant. Veja [Erros de Domínio](/guide-installation-shopify.html#shopify-domain-errors).

### Adicionando o FastComments a outras páginas

O mesmo snippet funciona em qualquer template Liquid, incluindo páginas de produtos, páginas personalizadas e a página inicial. Cole-o onde quiser que os comentários apareçam e ajuste `urlId` se desejar um identificador estável por página (por exemplo, `urlId: "{{ product.id }}"` em um template de produto).