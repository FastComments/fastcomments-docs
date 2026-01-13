Vermutlich möchten Sie die Konfiguration nicht inline definieren, wenn Sie Callbacks usw. übergeben. Stattdessen sollten Sie
die Konfiguration in einem `computed`-Block definieren, ansonsten wird jedes Mal, wenn Ihr Callback usw. aufgerufen wird, das gesamte Widget neu gerendert.

[Siehe das Spinner-Beispiel, um zu sehen, wie das geht.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)