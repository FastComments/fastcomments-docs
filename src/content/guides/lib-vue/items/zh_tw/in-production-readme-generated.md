你大概不想在行內定義 config，如果你要傳遞 callbacks 等。相反，你會想要定義
config 在一個 `computed` 區塊中，否則每次你的 callback 等被呼叫時，整個 widget 都會重新渲染。

[請參閱 spinner 範例以了解如何執行此操作。](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)