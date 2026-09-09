FastComments oferece suporte a webhooks apenas para o recurso Comentário.

Oferecemos suporte a webhooks para criação, remoção e atualização de comentários.

Cada um desses é considerado um evento separado em nosso sistema e, como tal, possui semânticas e estruturas diferentes para os eventos de webhook.

Qualquer número de endpoints pode se inscrever no mesmo evento: um webhook por domínio pode ser configurado no painel, e inscrições adicionais podem ser criadas através da API (veja Gerenciando Webhooks via API).