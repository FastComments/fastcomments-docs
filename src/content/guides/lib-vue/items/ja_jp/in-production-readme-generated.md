コールバック等を渡している場合、config をインラインで定義するのはおそらく望ましくありません。代わりに、`computed` ブロック内で config を定義してください。そうしないと、コールバック等が呼び出されるたびにウィジェット全体が再レンダリングされます。

[この方法についてはスピナーの例を参照してください。](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)