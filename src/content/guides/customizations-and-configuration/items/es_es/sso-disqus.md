La mayor diferencia entre Disqus y FastComments Secure SSO es que Disqus usa SHA1 para el cifrado mientras que nosotros usamos SHA256.

Esto significa que migrar desde Disqus es fácil - cambie el algoritmo de hash usado de SHA1 a SHA256 y actualice los nombres de las propiedades pasadas a la UI.