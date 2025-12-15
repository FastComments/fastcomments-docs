Você pode descobrir que nossas métricas de Analytics mostram números ligeiramente diferentes do que, digamos, Google Ads © ou produtos similares.

Para sites com um widget de comentários por página, os números fornecidos pelo FastComments Analytics são muito precisos, e se incorretos serão **menores** que o valor real, mas não maiores.

Se você tiver uma SPA, pode descobrir que os números do FastComments Analytics são mais altos do que os reportados por seus produtos de marketing. Isso ocorre porque o produto de marketing pode estar rastreando apenas quando a página não está carregada, mas não toda vez que um usuário faz algo na página que pode acionar a exibição de um novo thread de comentários, o que contaria como um carregamento de página para o FastComments Analytics.

#### Informações técnicas

FastComments Analytics rastreia cada carregamento de página e não depende de aleatoriedade como otimização. Cada carregamento de página resulta em uma atualização de contagem em memória em cada thread em cada servidor, que é então periodicamente persistida no banco de dados via uma operação atômica.
