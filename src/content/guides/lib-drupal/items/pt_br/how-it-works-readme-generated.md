---
Quando um usuário visita uma entidade com o campo FastComments habilitado:

1. O widget JavaScript do FastComments é carregado a partir do CDN.
2. Se o SSO estiver configurado, a identidade Drupal do usuário é passada para o FastComments.
3. Um fallback `<noscript>` fornece comentários renderizados no servidor para usuários sem JavaScript (apenas nos modos Live Comments e Streaming Chat).
---