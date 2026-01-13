---
Para desenvolvedores que você talvez não queira que sejam `Administrators`, considere criar um usuário `Administrator` com as seguintes permissões:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Esse conjunto de permissões dará a um desenvolvedor tudo que ele precisa para configurar o FastComments, além da visibilidade no sistema necessária para garantir que ele esteja funcionando.

A justificativa para essas permissões é a seguinte:

1. **Analytics Admin**: Isso pode ser usado para depurar o uso do FastComments.
2. **Customizations Admin**: Isso será necessário para aplicar estilização personalizada ao widget de comentários.
3. **Data Management Admin**: Isso será necessário para gerenciar importações e exportações, e configurar webhooks.
4. **Comment Moderation Admin**: Isso será necessário para visualizar os dados dos comentários, ao menos durante a configuração.
5. **API/SSO Admin**: Isso permitirá que eles obtenham as API keys diretamente da nossa plataforma. Consideramos isso mais seguro do que um `Administrator` copiá-las para eles e enviar o API Secret por e-mail, o que pode não ser muito seguro.
---