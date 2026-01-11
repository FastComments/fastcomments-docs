Siga os mesmos passos para `localhost` que seguiria para produção. Certifique-se de ter domínios de produção e API Secrets configurados.

Primeiro, navegue até o [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Isso é acessível em Gerenciar Dados -> Webhooks.

A página de configuração aparece da seguinte forma:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

Nesta página você pode especificar endpoints para cada tipo de evento de comentário.

Para cada tipo de evento, certifique-se de clicar em Enviar Payload de Teste para garantir que você configurou sua integração corretamente. Veja a próxima seção, "Testes", para detalhes.