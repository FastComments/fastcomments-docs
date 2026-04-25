Para a maioria dos sites, a maneira mais fácil de adicionar comentários é anexar o campo `FastComments` aos seus tipos de conteúdo. Vá para `Structure > Content types > [type] > Manage fields` e adicione o campo.

Cada entidade que tiver o campo recebe:

- Um **alternador de status** para que os editores possam ativar ou desativar os comentários por entidade.
- Um **identificador personalizado opcional** para que você possa usar um ID estável que não está vinculado ao caminho da entidade do Drupal.

O bloco principal `FastComments Widget` reconhece esse campo e ignorará entidades que já o tenham anexado. Assim, você pode misturar comentários por entidade com o bloco sem ver o widget duas vezes na mesma página.