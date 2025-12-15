Puede encontrar que nuestras métricas de Analytics muestran números ligeramente diferentes que, digamos, Google Ads © o productos similares.

Para sitios con un widget de comentarios por página, los números proporcionados por FastComments Analytics son muy precisos, y si son incorrectos serán **menores** que el valor real, pero no más.

Si tiene una SPA, puede encontrar que los números de FastComments Analytics son más altos que los informados por sus productos de marketing. Esto se debe a que el producto de marketing puede estar rastreando solo cuando la página no está cargada, pero no cada vez que un usuario hace algo en la página que podría activar la aparición de un nuevo hilo de comentarios, lo cual contaría como una carga de página para FastComments Analytics.

#### Información técnica

FastComments Analytics rastrea cada carga de página y no depende de la aleatoriedad como optimización. Cada carga de página resulta en una actualización de conteo en memoria en cada hilo en cada servidor, que luego se persiste periódicamente en la base de datos mediante una operación atómica.
