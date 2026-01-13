Вероятно, вы не захотите определять config inline, если передаёте callbacks и т. п. Вместо этого стоит определить config в блоке `computed`, иначе каждый раз при вызове вашего callback и т. п. весь виджет будет перерисовываться.

[См. пример spinner, чтобы узнать, как это сделать.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)