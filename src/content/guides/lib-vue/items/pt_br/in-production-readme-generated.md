Você provavelmente não vai querer definir o config inline se estiver passando callbacks etc. Em vez disso, você vai querer definir
o config em um bloco `computed`, caso contrário cada vez que seu callback etc. for invocado o widget inteiro será re-renderizado.

[Veja o exemplo do spinner para saber como fazer isso.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)