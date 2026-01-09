Les principales différences entre Disqus et FastComments Secure SSO sont que Disqus utilise SHA1 pour le chiffrement tandis que nous utilisons SHA256. 

Cela signifie que la migration depuis Disqus est simple - changez l'algorithme de hachage utilisé de SHA1 à SHA256 et mettez à jour les noms des propriétés transmis à l'UI.