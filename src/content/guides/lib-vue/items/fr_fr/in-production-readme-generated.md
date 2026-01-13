Vous ne voulez probablement pas définir la configuration en ligne si vous passez des callbacks, etc. À la place, vous devriez définir
la configuration dans un bloc `computed`, sinon chaque fois que votre callback, etc., sera invoqué, l'ensemble du widget sera rendu à nouveau.

[Voir l'exemple du spinner pour savoir comment faire.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)