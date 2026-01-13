FastComments oferece um modo de manutenção automático. Se o banco de dados ficar offline, ele pode continuar servindo tópicos de comentários populares.

Além disso, no modo de manutenção, todos os comentários são salvos em `BACKUP_DIR`. Eles serão processados (verificados quanto a spam, etc.) e salvos assim que o sistema voltar online.

Isso é feito determinando, a cada hora, os 100 tópicos de comentários mais populares e armazenando em cache seu conteúdo no disco. A determinação dos 100 principais tópicos já é feita a partir de um estado pré-calculado, portanto não é um trabalho periódico pesado.

Isso é completamente opcional e só é habilitado se `CACHE_DIR` e `BACKUP_DIR` estiverem configurados. Isso, é claro, torna os nós da aplicação com estado, no entanto é um estado que pode ser perdido a qualquer momento sem causar mau funcionamento da aplicação.

Observe que, no modo de manutenção, a autenticação adequada dos tópicos de comentários não pode ser realizada, então apenas tópicos que são seguramente considerados públicos são periodicamente salvos em backup.

No modo de manutenção, muitos recursos não estão disponíveis.