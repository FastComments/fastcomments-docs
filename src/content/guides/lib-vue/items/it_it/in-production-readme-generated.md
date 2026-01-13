Probabilmente non vuoi definire la config inline se stai passando callback, ecc. Invece, dovresti definire la config in un blocco `computed`, altrimenti ogni volta che il tuo callback, ecc., viene invocato l'intero widget verrà renderizzato di nuovo.

[Guarda l'esempio dello spinner per vedere come farlo.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)